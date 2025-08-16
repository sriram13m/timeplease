# timeplease

Rusty time conversion command line utility

## Installation

### From Source
```bash
git clone https://github.com/sriram13m/timeplease
cd timeplease
cargo build --release
sudo cp target/release/timeplease /usr/local/bin/
```

### Using Cargo
```bash
cargo install timeplease
```

## Usage

### Get Current Time
```bash
timeplease current london
```

**Output:**
```
Current time in London is 02:23 PM GMT, Monday, January 16, 2024
```

### Convert Time Between Timezones
```bash
timeplease convert 10:00 india london
```

**Output:**
```
10:00 Monday, January 16, 2024 India in London is 05:30 AM GMT, Monday, January 16, 2024
```

## Supported Cities

Major cities: london, mumbai, delhi, tokyo, nyc, la, sydney, paris, berlin, toronto, singapore, dubai

Country aliases: india, uk, usa, japan, australia, germany, france, canada

IANA codes: Europe/London, Asia/Kolkata, America/New_York, etc.
