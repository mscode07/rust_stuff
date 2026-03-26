use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len()<2{
        println!("Usage: add/list");
        println!("args: {:?}",args.len());
        return;
    }
    println!("args: {:?}",args);
    let command = &args[1];
    let mut tasks: Vec<String> = Vec::new();

    if command =="add"{
        if args.len() < 3{
            println!("Please Provide a Task to add");
            return;
        }
        let task = &args[2];
        tasks.push(task.to_string());
        println!("Task added: {}", task);
    } else if command == "list"{  
        for (i,task) in tasks.iter().enumerate(){
            println!("{}: {}",i + 1, task);
        }

    }
}