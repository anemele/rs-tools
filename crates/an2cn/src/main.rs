use argh::FromArgs;
use chinese_number::{ChineseCase, ChineseVariant};
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

#[derive(Debug, FromArgs)]
/// convert Arabic number to Chinese number
struct Args {
    #[argh(switch, short = 'l')]
    /// use lower case
    lower: bool,
}

fn main() {
    let args: Args = argh::from_env();

    let case = if args.lower {
        ChineseCase::Lower
    } else {
        ChineseCase::Upper
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
