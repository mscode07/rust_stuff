use crate::crypto::ssh;
pub fn generate(name:String){
println!("Generating the Key for {}",name);

let(private_key,public_key) = ssh::generate_ed25519_key(&name);
println!("\n Key Generated Successfully!");

println!("🔓 Public Key:\n{}\n", public_key);

println!("⚠️ Private Key (TEMP — not stored yet):\n{}\n", private_key);


}

pub fn list(){
    println!("Listing all the Keys.");
}

pub fn get(name:String,public:bool){
if public{
    println!("Getting public key for {}",name);
} else{
    println!("Getting private key for {}",name);
}
}