use regex::Regex;
use std::process::{Command, Output};
use clap::Parser;

use crate::cli::Cli;
// mod cli::Cli;
pub struct Diff {
    _files: Vec<DiffFile>,
    _new_lines: Vec<String>,
    _cagadas: Vec<Cagada>
}

impl Diff {
    pub fn get () /* -> Self */ {
        let args: Cli = Cli::parse();
        let mut command: Command = Command::new("git");

        command.arg("diff");
        if args.staged {
            command.arg("--staged");
        }

        let _output: Output = command.output().expect("failed to execute process");

        let files: Vec<DiffFile> = DiffFile::get();
        println!("{:?}", files);
        // let stdout = String::from_utf8(output.stdout).unwrap();
        // Diff {}
    }
}

#[derive(Debug)]
struct DiffFile {
    name: String,
    new: bool,
    deleted: bool,
    modified: bool,
}

impl DiffFile {
    fn get() -> Vec<DiffFile> {
        // let pattern: &str = r"diff.--git.a/(.*?) b/";
        // let re: Regex = Regex::new(r"([MDA])\\t(.*)").unwrap();
        let re: Regex = Regex::new(r":100644 100644 b0a8caa 0000000 M\tsrc/main.rs\n").unwrap();
        let std = re.captures(r"M\t").unwrap();
        let text: String =  DiffFile::raw_data();
        println!("{:?}", text);
        let mut res: Vec<DiffFile> = vec![];
        println!("{:?}", re.captures(&text).unwrap_or(std));

        // for captures in re.captures_iter(&text) {
        //     let name = captures.get(1).unwrap().as_str();
        //     res.push(DiffFile {
        //         name: name.to_owned(),
        //         new: false,
        //         deleted: false,
        //         modified: false,
        //     })
        // }

        res.push(DiffFile {
            name: "aa".to_owned(),
            new: false,
            deleted: false,
            modified: false,
        });
        res
    }

    fn raw_data() -> String {
        let args: Cli = Cli::parse();
        let mut command: Command = Command::new("git");
        command.arg("diff");
        command.arg("--raw");
        if args.staged {
            command.arg("--staged");
        }
        let output: Output = command.output().unwrap();
        String::from_utf8(output.stdout).unwrap_or("".to_owned())
    }
}

struct Cagada {
    _line: i32,
    _file: DiffFile,
    _regex_capture: String,
}
