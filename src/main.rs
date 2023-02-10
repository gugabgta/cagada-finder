use clap::Parser;
use crate::{cli::Cli, git::*};
use std::{fs, process::exit};

mod git;
mod cli;

fn main() {
    let args: Cli = Cli::parse();
    let mut diff = vec![];

    if args.input.is_none() {
        let mut com = Cli::git_diff_command();
        let files = DiffFile::get(&mut com).unwrap();
        for file in files {
            if args.full {
                diff.push (Diff::full(file));
            } else {
                diff.push(Diff::new(file, Cli::git_diff_command()));
            }
        }
        pretty_print(&diff);
        exit(0x0100);
    }

    let file = DiffFile::from_str(&args.input.unwrap());
    if file_exists(&file.name) {
        if args.full {
            diff.push (Diff::full(file));
        } else {
            diff.push(Diff::new(file, Cli::git_diff_command()));
        }
        pretty_print(&diff);
        exit(0x0100);
    }

    let mut com = Cli::git_show_command();
    let files = DiffFile::get(&mut com).unwrap();
    for file in files {
        diff.push(Diff::new(file, Cli::git_show_command()));
    }

    pretty_print(&diff);
}

fn file_exists(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn pretty_print(diffs: &Vec<Diff>) {
    for diff in diffs {
        if diff.cagadas.is_none() {
            continue;
        }
        for cagada in diff.cagadas.as_ref().unwrap() {
            println!("{}:{} {}", diff.file.name, cagada.line_number, cagada.line);
        }
    }
}
