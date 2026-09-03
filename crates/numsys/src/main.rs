use std::io::Read;

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

    let mut buf = String::new();
    if let Err(e) = std::io::stdin().lock().read_to_string(&mut buf) {
        eprintln!("{}", e);
        return;
    }

    if args.to_base == args.from_base {
        println!("{}", buf);
        return;
    }

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

    for line in buf.lines() {
        match ns._convert(line.trim(), args.from_base, args.to_base) {
            Ok(s) => println!("{}", s),
            Err(e) => {
                // stdout not stderr
                println!("{}", e)
            }
        }
    }
}
