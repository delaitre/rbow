use clap::Parser;
use rbow::{config::Config, config::ConfigPath, stylize::Stylize};
use std::{error::Error, io, io::prelude::*};

/// A simple program colorizing its input based on regular expressions.
///
/// Regular expressions to match are loaded from `.rbow` files in parent directories.
///
/// Example usage: `cat my_log_file.txt | rbow`
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Print an example .rbow file
    #[arg(short, long)]
    example: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.example {
        println!("{}", Config::example().serialized().unwrap());
        return Ok(());
    }

    let config = Config::load(ConfigPath::new(".", ".rbow"));
    let rules = config.as_rules();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let stylized_line = rules.stylize(line.as_str());
        println!("{}", stylized_line.unwrap_or(line));
    }

    Ok(())
}
