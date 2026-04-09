use crate::crypto::ssh;

pub fn generate(name:String){
    println!("Generating key for: {}",name);
    let(private_key,public_key) = ssh::generate_ed25519_key;
}