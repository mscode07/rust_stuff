use std :: {env,fs};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
struct  Task{
    title: String,
}

fn load_tasks()-> Vec<Task>{
    let data: Result<String, std::io::Error> = fs :: read_to_string("tasks.json");
    match data {
        Ok(content)=>{
            serde_json :: from_str(&content).unwrap_or(Vec::new())
        }
        Err(_)=> Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>){
    let data = serde_json::to_string(tasks).unwrap();
    fs::write("task.json",data).expect("Unable to write file");
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Usage: add/list");
        return;
    }
    let command = &args[1];

    let mut tasks = load_tasks();

    if command == "add"{
        if args.len()<3{
            println!("Please provide a task!!");
            return
        }
        let task = Task{
            title: args[2].clone(),
        };
        tasks.push(task);
        save_tasks(&tasks);
        println!("Task Added!!")
    } else if command == "list"{
        for (i, task) in tasks.iter().enumerate(){
            println!("{}: {}",i+1, task.title)
        }
    }
}