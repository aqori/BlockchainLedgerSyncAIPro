// src/main.rs
/*
 * Main executable for BlockchainLedgerSyncAIPro
 */

use clap::Parser;
use blockchainledgersyncaipro::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainLedgerSyncAIPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
