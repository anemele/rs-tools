use std::{
    io::BufRead,
    ops::{Add, Div},
};

use clap::Parser;
use rust_decimal::Decimal;

/// awkx: number processor
#[derive(Debug, Parser)]
pub enum Cli {
    Sum,
    Max,
    Min,
    Avg,
    Mid,
}

fn main() {
    let cmd = Cli::parse();

    let data: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| Decimal::from_str_exact(line.trim()).unwrap_or_default())
        .collect();

    let result = match cmd {
        Cli::Sum => data.iter().fold(Decimal::ZERO, Decimal::add),
        Cli::Max => *data.iter().max().unwrap_or(&Decimal::ZERO),
        Cli::Min => *data.iter().min().unwrap_or(&Decimal::ZERO),
        Cli::Avg => data.iter().fold(Decimal::ZERO, Decimal::add) / Decimal::from(data.len()),
        Cli::Mid => {
            let mut data = data.clone();
            data.sort_unstable();
            if data.len() & 1 == 1 {
                data[data.len() >> 1]
            } else {
                let idx = data.len() >> 1;
                (data[idx] + data[idx - 1]).div(Decimal::TWO)
            }
        }
    };
    println!("{}", result);
}
