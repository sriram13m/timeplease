pub mod cli;

use cli::{Args, Commands};
use std::{fs};
use std::sync::OnceLock;
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveTime, TimeZone};
use chrono_tz::Tz;

static TIMEZONE_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

fn get_current_time_in_timezone(tz: Tz) -> Result<DateTime<Tz>, Box<dyn std::error::Error>> {
    let utc_now = Utc::now();
    let local_time = utc_now.with_timezone(&tz);
    Ok(local_time)
}

fn convert_time(time_str: &str, from_tz: Tz, to_tz: Tz) -> Result<DateTime<Tz>, Box<dyn std::error::Error>> {
    let naive_time = NaiveTime::parse_from_str(time_str, "%H:%M")?;

    let today = Utc::now().with_timezone(&from_tz).date_naive();

    let naive_datetime = today.and_time(naive_time);

    let source_datetime = from_tz.from_local_datetime(&naive_datetime).single().ok_or("Ambiguous Time")?;

    Ok(source_datetime.with_timezone(&to_tz))
}


fn load_timezone_mapping() -> Result<&'static HashMap<String, String>, Box<dyn std::error::Error>> {
  let map = TIMEZONE_MAP.get_or_init(|| {
      let timezone_data = fs::read_to_string("src/timezones.json").expect("Failed to read timezone file");
      let timezone_mapping: HashMap<String, String> = serde_json::from_str(&timezone_data).expect("Failed to parse timezone JSON");
      timezone_mapping
  });

  Ok(map)
}


fn parse_timezone(tz_name: &str) -> Result<Tz, Box<dyn std::error::Error>> {
    let timezone_mappings = load_timezone_mapping()?;

    let lowercase_name = tz_name.to_lowercase();

    if let Some(iana_name) = timezone_mappings.get(&lowercase_name) {
        return iana_name.parse::<Tz>().map_err(|e| e.into())
    }

    tz_name.parse::<Tz>().map_err(|e| e.into())
}

fn capitalize_first(s: &str) -> String {
  let mut chars = s.chars();
  match chars.next() {
      None => String::new(),
      Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
  }
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Commands::Current { timezone } => {
            let tz = parse_timezone(&timezone)?;
            let current_time = get_current_time_in_timezone(tz)?;
             println!("Current time in {} is {}",
                  capitalize_first(&timezone),
                  current_time.format("%I:%M %p %Z, %A, %B %d, %Y")
              );
        }
        Commands::Convert { time, from, to } => {
              let from_tz = parse_timezone(&from)?;
              let to_tz = parse_timezone(&to)?;
              let converted_time = convert_time(&time, from_tz, to_tz)?;

              let today = chrono::Utc::now().with_timezone(&from_tz);

              println!("{} {}, {}, {} in {} is {}",
                  time,
                  today.format("%A"),
                  today.format("%B %d, %Y"),
                  capitalize_first(&from),
                  capitalize_first(&to),
                  converted_time.format("%I:%M %p %Z, %A, %B %d, %Y")
              );
          }
    }
    Ok(())
}
