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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_capitalize_first_normal_case() {
        assert_eq!(capitalize_first("london"), "London");
        assert_eq!(capitalize_first("mumbai"), "Mumbai");
        assert_eq!(capitalize_first("new_york"), "New_york");
    }

    #[test]
    fn test_capitalize_first_edge_cases() {
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("a"), "A");
        assert_eq!(capitalize_first("A"), "A");
        assert_eq!(capitalize_first("london"), "London");
    }

    #[test]
    fn test_parse_timezone_known_cities() {
        // Test cities from your JSON
        assert!(parse_timezone("london").is_ok());
        assert!(parse_timezone("mumbai").is_ok());
        assert!(parse_timezone("tokyo").is_ok());
        assert!(parse_timezone("nyc").is_ok());
        assert!(parse_timezone("india").is_ok());
    }

    #[test]
    fn test_parse_timezone_case_insensitive() {
        assert!(parse_timezone("LONDON").is_ok());
        assert!(parse_timezone("London").is_ok());
        assert!(parse_timezone("LoNdOn").is_ok());
    }

    #[test]
    fn test_parse_timezone_direct_iana() {
        // Test direct IANA timezone names
        assert!(parse_timezone("Europe/London").is_ok());
        assert!(parse_timezone("Asia/Kolkata").is_ok());
        assert!(parse_timezone("America/New_York").is_ok());
    }

    #[test]
    fn test_parse_timezone_unknown() {
        assert!(parse_timezone("atlantis").is_err());
        assert!(parse_timezone("unknown_city").is_err());
        assert!(parse_timezone("").is_err());
    }

    #[test]
    fn test_get_current_time_in_timezone() {
        let london_tz = parse_timezone("london").unwrap();
        let result = get_current_time_in_timezone(london_tz);
        assert!(result.is_ok());
        
        let current_time = result.unwrap();
        // Basic sanity check - time should be reasonable
        assert!(current_time.year() >= 2024);
    }

    #[test]
    fn test_convert_time_basic() {
        let from_tz = parse_timezone("mumbai").unwrap();
        let to_tz = parse_timezone("london").unwrap();
        
        let result = convert_time("10:00", from_tz, to_tz);
        assert!(result.is_ok());
        
        let converted = result.unwrap();
        // Mumbai is UTC+5:30, London is UTC+0 (winter) or UTC+1 (summer)
        // So 10:00 in Mumbai should be 4:30 AM or 5:30 AM in London
        let hour = converted.hour();
        assert!(hour == 4 || hour == 5); // Account for daylight saving
    }

    #[test]
    fn test_convert_time_same_timezone() {
        let london_tz = parse_timezone("london").unwrap();
        
        let result = convert_time("15:30", london_tz, london_tz);
        assert!(result.is_ok());
        
        let converted = result.unwrap();
        assert_eq!(converted.hour(), 15);
        assert_eq!(converted.minute(), 30);
    }

    #[test]
    fn test_convert_time_invalid_format() {
        let from_tz = parse_timezone("london").unwrap();
        let to_tz = parse_timezone("mumbai").unwrap();
        
        // Invalid time formats
        assert!(convert_time("25:00", from_tz, to_tz).is_err()); // Invalid hour
        assert!(convert_time("10:60", from_tz, to_tz).is_err()); // Invalid minute
        assert!(convert_time("invalid", from_tz, to_tz).is_err()); // Not a time
        assert!(convert_time("", from_tz, to_tz).is_err()); // Empty string
    }

    #[test]
    fn test_convert_time_edge_hours() {
        let from_tz = parse_timezone("mumbai").unwrap();
        let to_tz = parse_timezone("london").unwrap();
        
        // Test edge cases
        assert!(convert_time("00:00", from_tz, to_tz).is_ok()); // Midnight
        assert!(convert_time("23:59", from_tz, to_tz).is_ok()); // Almost midnight
        assert!(convert_time("12:00", from_tz, to_tz).is_ok()); // Noon
    }

    #[test]
    fn test_multiple_timezone_lookups() {
        // Test that your OnceLock optimization works
        assert!(parse_timezone("london").is_ok());
        assert!(parse_timezone("mumbai").is_ok());
        assert!(parse_timezone("tokyo").is_ok());
        // Should only load JSON once thanks to OnceLock
    }

    #[test]
    fn test_timezone_abbreviations() {
        // Test common abbreviations from your JSON
        assert!(parse_timezone("nyc").is_ok());
        assert!(parse_timezone("la").is_ok());
        assert!(parse_timezone("uk").is_ok());
        assert!(parse_timezone("usa").is_ok());
    }
}
