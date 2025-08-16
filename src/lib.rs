pub mod cli;

use cli::{Args, Commands};

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Commands::Current { timezone } => {
            println!("Getting current time in {}", timezone);
            Ok(())
        }
        Commands::Convert { time, from, to } => {
            println!("Trying to convert {} from {} to {}", time, from, to);
            Ok(())
        }
    }
}
