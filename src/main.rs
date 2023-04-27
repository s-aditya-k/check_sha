#![warn(clippy::pedantic, clippy::style, clippy::nursery)]

use color_eyre::Result;
use clap::Parser;
use color_eyre::eyre::{eyre};
use owo_colors::OwoColorize;

fn main() -> Result<()> {
    color_eyre::install()?;
    let calculated_sum = input()?;
    let Cli { user_sum } = Cli::parse();
    if calculated_sum == user_sum {
        println!("{}", "Sum is correct!".bold());
    } else {
        println!("{} ", "Sum is not correct!!".bold());
    }
    Ok(())
}

fn input() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let mut iter = input.trim().split_whitespace();
    let sum = iter.next().ok_or_else(|| eyre!("did not pipe in input!"))?;
    Ok(sum.to_string())
}

#[derive(Parser)]
struct Cli {
    user_sum: String,
}