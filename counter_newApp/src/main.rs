use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::{Value, json};

#[derive(Debug)]
enum ApiError{
    NotFound,
    InvalidInput(String),
    InterError,
}

impl IntoResponse for ApiError {
     fn into_response(self) -> axum::response::Response {
         let (status, error_message) = match self {
             ApiError::NotFound => (
                StatusCode::NOT_FOUND, "Data not found".to_string(),
             ),
             ApiError::InvalidInput(msg) => (
                StatusCode::BAD_REQUEST,msg,
             ),
            ApiError::InterError =>(
                StatusCode::INTERNAL_SERVER_ERROR,"internal Error".to_string()
            )
         };
         let body = Json(json!({
            "error": error_message,
         }));
         (status,body).into_response()
     }
}

#[tokio::main]
async fn main(){
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind tcp listener"); 

    println!("Sever is running on http://localhost:3000");
        axum::serve(listener, app).await.unwrap();

}

fn create_app() -> Router{
    Router::new()
    .route("/health", get(health_check))
    .route("/users", get( list_users))
    .route("/user/{id}", get(get_user))
}

async fn health_check() -> impl IntoResponse{
    Json(json!({
        "status": "ok",
        "message":"Sever is running"
    }))
}

async fn list_users() -> Result<Json<Value>, ApiError>{
    Err(ApiError::InterError)
}

async fn get_user(Path(id): Path<u32>) -> Result<Json<Value>,ApiError>{
    if id > 100{
        return Err(ApiError::NotFound);
    }
    Ok(Json(json!({"id":id, "name":"user"})))
}