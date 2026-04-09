use ssh_key::{private::PrivateKey,LineEnding};
use rand::rngs::OsRng;

pub fn generate_ed25519_key()->(String,String){
    let mut rng = OsRng;

    let private_key = PrivateKey::random(&mut rng,ssh_key::Alogrithm::Ed25519)
    .expect("Failed to generate key");

    let private_key_str = private_key.to_openssh(LineEnding::LF)
    .expect("Faild to encode private key");

    let public_key = private_key.public_key();
    let public_key_str = private_key.to_string();

    (private_key_str, public_key_str)
}