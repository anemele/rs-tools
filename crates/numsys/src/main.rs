use argh::FromArgs;
use numsys::NumSys;
use std::io::BufRead;

#[derive(Debug, FromArgs)]
/// numeral system: convert number between different bases.
struct Args {
    #[argh(option, short = 't', default = "16")]
    /// to base, default 16
    to_base: usize,

    #[argh(option, short = 'f', default = "10")]
    /// from base, default 10
    from_base: usize,

    #[argh(option, short = 'c')]
    /// charset, default [0-9A-Z]
    charset: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();

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
