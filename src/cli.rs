use std::process::Command;
use clap::{Parser, arg};

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'p', long = "pattern")]
    pub pattern: Option<String>,

    #[arg(long)]
    pub staged: bool,

    #[arg(short = 'f', long = "full")]
    pub full: bool,

    #[clap(help = "Input file", index = 1)]
    pub input: Option<String>,
}

impl Cli {
    pub fn git_diff_command() -> Command {
        let args: Cli = Cli::parse();
        let mut command: Command = Command::new("git");
        command.arg("diff");
        if args.staged {
            command.arg("--staged");
        }

        command
    }

    pub fn git_show_command () -> Command {
        let mut command: Command = Command::new("git");
        command.arg("show");

        command
    }
}

