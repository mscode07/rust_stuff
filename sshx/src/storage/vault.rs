use std::{fs, path::PathBuf};

pub fn get_vault_path()->PathBuf{
    let mut path = dirs::home_dir().expect("Could not find home dir");

    path.push(".sshx");
    path.push("vault");
    path
}

pub fn init_vault(){
    let path = get_vault_path();

    if !path.exists(){
        fs::create_dir_all(&path).expect(
            "Failed to create vault dir"
        )
    }
}

pub fn save_key(name: &str, private_key: &str, public_key:&str){
    init_vault();
    let base_path = get_vault_path();

    let private_path = base_path.join(name);
    let public_path = base_path.join(format!("{}.pub", name));

     
    fs::write(&private_path, private_key).expect("Failed to save private key");
    fs::write(&public_path, public_key).expect("Failed to save public key");
}