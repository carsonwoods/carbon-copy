use std::fs;
use std::path::PathBuf;

use clap::Parser;
use clap::parser::ValueSource;

mod copy;
mod hash;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    source_file: PathBuf,

    target_file: PathBuf,

    /// Copy source_file recursively
    #[arg(short, long, default_value_t = false)]
    recursive: bool,

    /// Specify alternative hashing algorithm
    #[arg(long, default_value_t = String::from("md5"))]
    hash: String,
}

fn main() {
    // parse arguments
    let cli = Cli::parse();

    // ensures source_file exists
    if !cli.source_file.exists() {
        eprintln!("carbon: {:?}: No such file or directory", cli.source_file);
        std::process::exit(1);
    }

    // checks to see if recursive mode is enabled for a directory
    if cli.source_file.is_dir() {
        if !cli.recursive {
            eprintln!("carbon: {:?} is a directory (not copied).", cli.source_file);
            std::process::exit(1);
        } else {
            if let Err(e) = copy::copy_dir_recursive(&cli.source_file, &cli.target_file, &cli.hash)
            {
                eprintln!("carbon: {e}");
                std::process::exit(1);
            }
        }
    } else {
        copy::copy_file(&cli.source_file, &cli.target_file, &cli.hash);
    }
}
