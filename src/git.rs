use regex::{Regex, RegexBuilder, RegexSet};
use std::{process::{Command, Output}, io::{BufReader, BufRead}, fs::File};
use clap::Parser;
use crate::cli::Cli;

const DEFAULT_REGEX: &'static [&str] = &[
    r"^\s*//",
    r"^\s*/\*",
    r"^\s*\*\\",
];

const DEFAULT_GIT_REGEX: &'static [&str] = &[
    r"^\+[^\+]*//",
    r"^\+[^\+]* \s*/\*",
    r"^\+[^\+]* \s*\*\\",
];
pub struct Diff {
    files: Option<Vec<DiffFile>>,
    cagadas: Option<Vec<Cagada>>,
    cagada_count: i32,
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

        let files = DiffFile::get().unwrap();
        for file in files {
            Cagada::full(file);
        }
        // let stdout = String::from_utf8(output.stdout).unwrap();
        // Diff {}
    }
}

#[derive(Debug)]
struct DiffFile {
    name: String,
    extension: String,
    status: DiffFileStatus,
}

#[derive(Debug)]
enum DiffFileStatus {
    New,
    Deleted,
    Modified,
}

impl DiffFile {
    fn get() -> Option<Vec<DiffFile>> {
        let re: Regex = RegexBuilder::new(r"^.*([MDA])\W*(.*)$")
            .multi_line(true)
            .build()
            .unwrap();
        let text: String =  DiffFile::raw_data();

        let mut res: Vec<DiffFile> = vec![];

        for capture in re.captures_iter(&text) {
            let name = capture.get(2).unwrap().as_str().to_owned();
            let status: DiffFileStatus = match capture.get(1).unwrap().as_str() {
                "A" => DiffFileStatus::New,
                "D" => DiffFileStatus::Deleted,
                /* M */_ => DiffFileStatus::Modified,
            };
            let extension = extract_extension(&name);

            res.push(DiffFile {
                name,
                status,
                extension
            })
        }
        Some(res)
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
    line: i32,
    file: DiffFile,
    regex_capture: Regex,
}

impl Cagada {
    fn default_regex() -> RegexSet {
        RegexSet::new(DEFAULT_REGEX).unwrap()
    }

    fn full(dfile: DiffFile) {
        let file = File::open(&dfile.name).unwrap();
        let reader = BufReader::new(file);
        let re: RegexSet = Cagada::default_regex();
        let mut line_number: i32 = 0;

        for line in reader.lines() {
            line_number += 1;
            let pline: &str = &line.unwrap_or_default();
            if re.is_match(pline) {
                println!("{}:{} {}", dfile.name, line_number, pline);
            }
        }
    }

    fn git(dfile: DiffFile) {
        let args: Cli = Cli::parse();
        let mut command: Command = Command::new("git");
        command.arg("diff");
        if args.staged {
            command.arg("--staged");
        }
        command.arg(dfile.name);
        
        // let output: Output = command.output().unwrap();
        // let reader = BufReader::new(output);
        // let re: RegexSet = Cagada::default_regex();
        // println!("{:?}", output);
    }
}

fn extract_extension(file: &str) -> String {
    let re: Regex = Regex::new(r"\.(.*)").unwrap();
    match re.captures(file) {
        Some(cap) => cap.get(1).unwrap().as_str().to_owned(),
        None => "".to_owned()
    }
}
//
