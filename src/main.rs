use std::process::Command;

use crate::cli::Cli;

#[allow(unused_results)]
#[warn(unused_imports)]
#[warn(unused_variables)]
#[warn(dead_code)]

mod git;
mod cli;

fn main() {
    let command: Command = Cli::gitDiffCommand();
    git::Diff::get(command);
}
