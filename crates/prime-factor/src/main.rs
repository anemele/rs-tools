use argh::FromArgs;
use bytemuck::cast_slice;
use memmap2::Mmap;
use std::{
    fs,
    io::{self, BufRead, Write},
};

const N: usize = 100_000_000;

const CACHE_PATH: &str = ".cache/prime-factor";
const SPF_FILE: &str = "spf.bin";

fn get_spf_file(re_generate: bool) -> io::Result<(fs::File, Mmap)> {
    let cache_path = std::env::home_dir().unwrap_or_default().join(CACHE_PATH);
    if !fs::exists(&cache_path)? {
        fs::create_dir_all(&cache_path)?;
    }

    let spf_file = cache_path.join(SPF_FILE);
    if re_generate || !fs::exists(&spf_file)? {
        eprintln!("generating spf file...");
        let mut spf = vec![0u32; N];
        let mut prime = vec![0u32; 0];

        for i in 2..N {
            if spf[i] == 0 {
                spf[i] = i as u32;
                prime.push(i as u32);
            }
            for p in &prime {
                let p = *p;
                let ip = p as usize * i;
                if ip >= N {
                    break;
                }
                spf[ip] = p;
                if p == spf[i] {
                    break;
                }
            }
        }

        let mut file = fs::File::create(&spf_file)?;
        let bytes = cast_slice(&spf);
        file.write_all(bytes)?;
    }

    let file = fs::File::open(&spf_file)?;
    let mmap = unsafe { Mmap::map(&file)? };
    eprintln!("loaded spf file");

    Ok((file, mmap))
}

fn factor(view: &[u32], mut n: u32) -> Option<String> {
    if n <= 1 {
        return Some(n.to_string());
    }

    let mut sv = vec![];
    while n > 1 {
        let spf = *view.get(n as usize)?;
        let mut c = 1;
        n /= spf;
        while n.is_multiple_of(spf) {
            c += 1;
            n /= spf;
        }
        sv.push(if c == 1 {
            format!("{spf}")
        } else {
            format!("{spf}^{c}")
        });
    }
    Some(sv.join("*"))
}

#[derive(Debug, FromArgs)]
/// prime factor/filter
struct Args {
    #[argh(switch, short = 'g')]
    /// re-generate spf file
    re_generate: bool,

    #[argh(switch, short = 'f')]
    /// filter, not factor
    filter: bool,
}

fn main() {
    let args: Args = argh::from_env();

    let (_file, mmap) = match get_spf_file(args.re_generate) {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("failed to open spf file: {}", e);
            return;
        }
    };
    let view: &[u32] = cast_slice(&mmap);

    let nums = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| match line.trim().parse::<u32>() {
            Ok(n) => Some(n),
            Err(e) => {
                eprintln!("{}", e);
                println!();
                None
            }
        });

    if args.filter {
        nums.filter(|n| *n > 1 && view.get(*n as usize).is_some_and(|nn| nn == n))
            .for_each(|n| {
                println!("{}", n);
            });
    } else {
        nums.for_each(|n| match factor(view, n) {
            Some(s) => println!("{}", s),
            None => {
                eprintln!("over max: {}", N);
                println!()
            }
        })
    }
}
