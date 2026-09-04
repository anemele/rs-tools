use clap::Parser;
use crypt_file::cmd::{decrypt, encrypt, glob_files};

#[derive(Debug, Parser)]
#[clap(
    name = "cf",
    author,
    version,
    about = "Crypt File Tool",
    long_about = "A file crypto tool based on xor alg, use randomly generated key."
)]
enum Cli {
    /// encrypt file, alias: e
    #[clap(visible_alias = "e", about = "encrypt file")]
    Encrypt {
        #[arg(required = true, help = "file or path, glob support")]
        args: Vec<String>,
    },

    /// decrypt file, alias: d
    #[clap(visible_alias = "d", about = "decrypt file")]
    Decrypt {
        #[arg(required = true, help = "file or path, glob support")]
        args: Vec<String>,
    },

    /// glob file, alias: g
    #[clap(visible_alias = "g", about = "test glob")]
    Glob {
        #[arg(required = true, help = "file or path, glob support")]
        args: Vec<String>,
    },
}

fn main() {
    match Cli::parse() {
        Cli::Encrypt { args } => {
            for file in glob_files(&args) {
                match encrypt(&file) {
                    Ok(n) => println!("{n}"),
                    Err(e) => eprintln!("{e}"),
                }
            }
        }
        Cli::Decrypt { args } => {
            for file in glob_files(&args) {
                match decrypt(&file) {
                    Ok(n) => println!("{n}"),
                    Err(e) => eprintln!("{e}"),
                }
            }
        }
        Cli::Glob { args } => {
            for p in glob_files(&args) {
                println!("{}", p.display());
            }
        }
    }
}
