use clap::{Subcommand, Parser};

#[derive(Parser)]
#[command(name = "Pipeline")]
#[command(about = "Concurrent File Processing")]
struct CLI {
    #[command(subcommand)]
    command : Commands,
}

#[derive(Subcommand)]
enum Commands {

    Process {
        path : String,
    },
    
}

fn main() 
{
    let cli = CLI::parse();

    match cli.command {
        Commands::Process { path } => {
            println!("Would Process Directory {}", path);
        }
    }
}