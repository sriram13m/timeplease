pub mod cli;

use cli::{Args, Commands};
use std::fs;
use std::collections::HashMap;
use chrono_tz::Tz;

fn load_timezone_mapping() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let timezone_data = fs::read_to_string("src/timezones.json")?;

    let timezone_mapping : HashMap<String, String> = serde_json::from_str(&timezone_data)?;

    Ok(timezone_mapping)
}

fn parse_timezone(tz_name: &str) -> Result<Tz, Box<dyn std::error::Error>> {
    let timezone_mappings = load_timezone_mapping()?;

    let lowercase_name = tz_name.to_lowercase();

    if let Some(iana_name) = timezone_mappings.get(&lowercase_name) {
        return iana_name.parse::<Tz>().map_err(|e| e.into())
    }

    tz_name.parse::<Tz>().map_err(|e| e.into())
}

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
