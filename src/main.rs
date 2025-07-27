// src/main.rs
/*
 * Main executable for ModernHyperion
 */

use clap::Parser;
use modernhyperion::{Result, run};

#[derive(Parser)]
#[command(version, about = "ModernHyperion - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
