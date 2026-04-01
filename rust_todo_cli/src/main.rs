use clap::{Parser, Subcommand};
use std::{env, fs};
use serde::{Serialize,Deserialize};

#[derive(Parser)]
#[command(name="todo")]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Add{task:String},
    List
}
#[derive(Serialize,Deserialize)]
struct Task{
    Title: String,
}

fn save_tasks(tasks: &Vec<Task>){
    let data = serde_json::to_string(tasks).unwrap();
    fs::write("task_.json", data).expect("Unable to write file");
}

fn load_tasks()->Vec<Task>{
    let data = fs::read_to_string("task_.json");
    match data {
        Ok(content)=>{
            serde_json ::from_str(&content).unwrap_or(Vec::new())
        }
        Err(_)=>Vec::new(),
    }
}

fn main(){
    let cli = Cli::parse();
    let mut tasks=load_tasks();

    match cli.command {
        Commands::Add { task }=>{
            tasks.push(Task{Title:task});
            save_tasks(&tasks);
            println!("Task added!!");
        }

        Commands::List=>{
            if tasks.is_empty(){
                println!("No task Found!!");
            }else{
                for (i, task) in tasks.iter().enumerate(){
                    println!("{}:{}", i+1, task.Title)
                }
            }
        }
    }
}