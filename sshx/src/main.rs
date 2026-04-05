use clap::{Parser,Subcommand};
#[derive(Parser)]
#[command(name="sshx")]
#[command (about="Secure SSH Key manager",long_about=None)]

struct Cli{
    #[command(subcommand)]
    command: Commands
}
#[derive(Subcommand)]
enum Commands {
    Generate{
        #[arg(short,long)]
        name:String,
    },
    List,
    Get{
        name:String,
        #[arg(long)]
        public: bool
    }
}
fn main() {
    println!("Hello, world!");
    println!("Testing");
}
