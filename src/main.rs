use rbow::{config::Config, config::ConfigPath, stylize::Stylize};
use std::{error::Error, io, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load(ConfigPath::new(".", ".rbow"));
    //let config = Config::example();
    let rules = config.as_rules();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let stylized_line = rules.stylize(line.as_str());
        println!("{}", stylized_line.unwrap_or(line));
    }

    Ok(())
}
