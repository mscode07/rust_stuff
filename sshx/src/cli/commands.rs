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
    println!("Listing all the Keys.");
}

pub fn get(name:String,public:bool){
if public{
    println!("Getting public key for {}",name);
} else{
    println!("Getting private key for {}",name);
}
}