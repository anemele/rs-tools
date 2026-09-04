use std::io::BufRead;

use clap::Parser;
use numsys::NumSys;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value_t = 16)]
    to_base: usize,

    #[arg(default_value_t = 10)]
    from_base: usize,

    #[arg(help = "default: 0-9A-Z")]
    charset: Option<String>,
}

fn main() {
    let args = Args::parse();

    let ns = match args.charset {
        None => NumSys::default(),
        Some(s) => match NumSys::new(&s) {
            Ok(ns) => ns,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        },
    };
    if let Err(e) = ns.check_base(args.from_base, args.to_base) {
        eprintln!("{}", e);
        return;
    }

    std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .for_each(
            |line| match ns._convert(line.trim(), args.from_base, args.to_base) {
                Ok(s) => println!("{}", s),
                Err(e) => {
                    eprintln!("error: {}", e);
                    println!()
                }
            },
        );
}
