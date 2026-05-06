use crate::crypto::ssh;
use crate::storage::vault;

pub fn generate(name:String){
println!("Generating the Key for {}",name);

let(private_key,public_key) = ssh::generate_ed25519_key(&name);

vault::save_key(&name, &private_key, &public_key);

 println!("\n✅ Key saved successfully!");

    println!("\n🔓 Public Key:\n{}\n", public_key);


}

pub fn list(){
    let keys = vault::list_keys();

    if keys.is_empty(){
        println!("No keys found");
        return;
    }
    println!("Stored keys:\n");
    
    for key in keys{
        println!("- {}",key);
    }
}

pub fn get(name:String,public:bool){
let key = vault::get_key((&name), public);

println!("{}",key);
}
pub fn delete(name: String) {
    vault::delete_key(&name);

    println!("✅ Deleted key '{}'", name);
}