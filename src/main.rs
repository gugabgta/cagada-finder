use clap::Parser;

use crate::cli::Cli;

mod git;
mod cli;

fn main() {
    // let command: Command = Cli::gitDiffCommand();
    let args: Cli = Cli::parse();
    let full = args.full;
    
    // let diff = match full {
        // true => git::Diff::full(),
        // false => git::Diff::new(),
    // };
}
