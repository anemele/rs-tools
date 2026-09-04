use chinese_number::{ChineseCase, ChineseVariant};
use clap::{Parser, ValueEnum};
use std::io::BufRead;

fn convert_number(num: &str, case: ChineseCase) -> String {
    if let Ok(n) = num.parse::<i64>() {
        return chinese_number::from_i64_to_chinese_high(ChineseVariant::Simple, case, n);
    }

    if let Ok(n) = num.parse::<f64>() {
        return match chinese_number::from_f64_to_chinese_ten_thousand(
            ChineseVariant::Simple,
            case,
            n,
        ) {
            Ok(s) => s,
            Err(e) => format!("{e}"),
        };
    }

    "not a number".into()
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Case {
    Upper,
    Lower,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_enum, default_value_t=Case::Upper)]
    case: Case,
}

fn main() {
    let args = Args::parse();

    let case = match args.case {
        Case::Upper => ChineseCase::Upper,
        Case::Lower => ChineseCase::Lower,
    };

    std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .for_each(|line| {
            let s = convert_number(line.trim(), case);
            println!("{}", s)
        });
}
