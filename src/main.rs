use colored::Color;
use rbow::{rule::Rule, stylize::Stylize};
use std::{error::Error, io, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let warning_rule = Rule::new(r"([Ww][Aa][Rr][Nn][Ii][Nn][Gg])", vec![Color::Yellow])?;
    let error_rule = Rule::new(r"([Ee][Rr][Rr][Oo][Rr])", vec![Color::Red])?;

    let rules = vec![warning_rule, error_rule];

    for line in io::stdin().lock().lines() {
        let line = line?;
        let stylized_line = rules.stylize(line.as_str());
        println!("{}", stylized_line.unwrap_or(line));
    }

    Ok(())
}
