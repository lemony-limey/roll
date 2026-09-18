use std::{fmt, ops::Deref};

use clap::{Parser, ValueEnum};
use rand::RngExt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli
{
    /// The die variant to simulate rolling.
    #[arg(short, long, required = true)]
    die: Die,
    
    /// The number of dice rolled.
    #[arg(short, long, required = true)]
    number_of_dice_rolled: usize,

    /// The proficiency modifier to be added to the roll, if any.
    #[arg(short, long, required = false, default_value = "0")]
    proficiency_modifier: usize,

    /// Any additional constant bonuses added to the roll (not including proficiency modifier).
    #[arg(short, long, required = false, default_value = "0")]
    additional: usize,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Die
{
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
}

impl Die
{
    fn roll(&self) -> u8
    {
        rand::rng().random_range(1..=(*self as u8))
    }
}

fn calculate(args: &Cli) -> usize
{
    // Perform all rolls.
    let mut rolls = 0;
    for _ in 0..args.number_of_dice_rolled
    {
        let roll = args.die.roll() as usize;
        println!("debug: roll = {}", roll);
        rolls += roll;
    }

    // Add any bonuses.
    let result = rolls
        + args.proficiency_modifier
        + args.additional;

    println!(
        "{} + {} = {}",
        rolls,
        (&args.additional + &args.proficiency_modifier),
        result,
    );
    
    result
}

impl fmt::Display for Die
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        writeln!(f, "d{}", *self as u8)
    }
}

fn main() -> anyhow::Result<()>
{
    let args = Cli::parse();

    println!(
        "rolling {}{}",
        &args.number_of_dice_rolled,
        &args.die,
    );

    let result = calculate(&args);
    
    println!("--- result = {} ---", result);
    
    Ok(())
}
