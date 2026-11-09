use clap::{Subcommand, Parser};
use std::fs;

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

fn main() -> std::io::Result<()> 
{
    let cli = CLI::parse();

    let Commands::Process { path } = cli.command;
    process_directory(&path)?;

    Ok(())

}
fn process_directory(dir : &str) -> std::io::Result<()> {

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();    

        if path.is_file() {
            let size = entry.metadata()?.len();
            println!("{}  ({} bytes)", path.display(), size);
        }

        else {
            println!("{}", path.display())
        }
    }
    Ok(())

}
