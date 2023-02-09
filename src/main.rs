use clap::Parser;

use crate::cli::Cli;

mod git;
mod cli;

fn main() {
    let args: Cli = Cli::parse();
    let full = args.full;

    let diff = match full {
        true => git::Diff::full(),
        false => git::Diff::new()
    };

    println!("{} Cagadas foram encontradas!", diff.cagada_count);
    for cagada in diff.cagadas.iter().flatten() {
        println!("{}", cagada.format());
    }
}
