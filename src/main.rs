use timeplease::cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    if let Err(e) = timeplease::run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

}
