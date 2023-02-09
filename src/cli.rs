use std::process::Command;

use clap::{Parser, arg};

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'p', long = "pattern", default_value ="")]
    pub pattern: String,

    #[arg(long)]
    pub staged: bool,

    #[arg(short = 'f', long = "full")]
    pub full: bool,
}

impl Cli {
    pub fn git_diff_command()-> Command {
        let args: Cli = Cli::parse();
        let mut command: Command = Command::new("git");
        command.arg("diff");
        if args.staged {
            command.arg("--staged");
        }

        command
    }
}
