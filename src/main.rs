// src/main.rs
/*
 * Main executable for DigitalOwnership
 */

use clap::Parser;
use digitalownership::{Result, run};

#[derive(Parser)]
#[command(version, about = "DigitalOwnership - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
