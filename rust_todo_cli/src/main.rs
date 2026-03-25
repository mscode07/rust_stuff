use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len()<2{
        println!("Usage: add/list");
        return;
    }
    let command = &args[1];

    if command == "add"{
        let task = &args[2];
        println!("Adding Task: {}",task);
    } else if command == "list"{
        println!("Listing Tasks...");
    } else {
        println!("unknown command");
    }
}