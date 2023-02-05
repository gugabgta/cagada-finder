use regex::{Regex, RegexBuilder, RegexSet};
use std::{
    process::{
        Command, Output
    }, 
    io::{
        BufReader, 
        BufRead
    }, 
    fs::File
};
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

const GIT_LINE_NUMBER_REGEX: &str = r"@@ -\d+,\d+ \+(\d+)";
const GIT_NEW_LINE_REGEX: &str = r"^\+[^\+].*";
const GIT_REMOVED_LINE_REGEX: &str = r"^\-[^\-]*?";

pub struct Diff {
    files: Option<Vec<DiffFile>>,
    cagadas: Option<Vec<Cagada>>,
    cagada_count: i32,
}

impl Diff {
    fn files() -> Vec<DiffFile> {
        DiffFile::get().unwrap()
    }

    pub fn new () /* -> Self */ {
        let files = Diff::files();
        let mut cagadas: Vec<Cagada> = vec![];
        for file in files {
            // cagadas.push(Cagada::git(file));
        };
    }

    // pub fn full () -> Self {
        // let files = Diff::files();
        // for file in files {
        //     Cagada::full(file);
        // }

        // let stdout = String::from_utf8(output.stdout).unwrap();
        // Diff {}
    // }
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
    Undefined,
}

impl DiffFile {
    fn get() -> Option<Vec<DiffFile>> {
        let re: Regex = RegexBuilder::new(r"^.*([MDA])\W*(.*)$")
            .multi_line(true)
            .build()
            .unwrap();
            
        let mut command: Command = Cli::gitDiffCommand();
        command.arg("--raw");

        let output: Output = command.output().unwrap();
        let text: String = String::from_utf8(output.stdout).unwrap_or("".to_owned());

        let mut res: Vec<DiffFile> = vec![];

        for capture in re.captures_iter(&text) {
            let name = capture.get(2).unwrap().as_str().to_owned();
            let status: DiffFileStatus = match capture.get(1).unwrap().as_str() {
                "A" => DiffFileStatus::New,
                "D" => DiffFileStatus::Deleted,
                "M" => DiffFileStatus::Modified,
                _ => DiffFileStatus::Undefined,
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

    fn from_str(file_name: &str) -> Self {
        DiffFile { 
            name: file_name.to_owned(), 
            extension: extract_extension(&file_name), 
            status: DiffFileStatus::Undefined,
        }
    }
}

struct Cagada {
    line_number: i32,
    line: String,
    file: DiffFile,
}

impl Cagada {
    fn default_regex() -> RegexSet {
        RegexSet::new(DEFAULT_REGEX).unwrap()
    }

    fn full(&self, dfile: DiffFile) {
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

    fn git(dfile: DiffFile) -> Vec<Cagada> {
        let mut command = Cli::gitDiffCommand();
        command.arg(&dfile.name);
        let mut parser = GitParser { command };
        parser.parse_command(&dfile.name)
    }
}

struct GitParser {
    command: Command,
}

impl GitParser {
    fn parse_command(&mut self, file_name: &str) -> Vec<Cagada> {
        let output: Output = self.command.output().unwrap();
        let lines = String::from_utf8(output.stdout).unwrap_or("".to_owned());
        let split = lines.split("\n");
        let vec = split.collect::<Vec<&str>>();
        let mut line_number = 0;
        let line_re: Regex = Regex::new(GIT_LINE_NUMBER_REGEX).unwrap();
        let new_line_re: Regex = Regex::new(GIT_NEW_LINE_REGEX).unwrap();
        let old_line_re: Regex = Regex::new(GIT_REMOVED_LINE_REGEX).unwrap();
        let re = RegexSet::new(DEFAULT_GIT_REGEX).unwrap();

        let mut res = vec![];
        for line in vec {
            if line_re.is_match(line) {
                line_number = line_re.captures(line).unwrap()
                    .get(1).unwrap()
                    .as_str().parse::<i32>().unwrap() - 1;
                }
            if re.is_match(line) {
                res.push(Cagada {
                    line: rem_first_letter(line).to_owned(),
                    line_number,
                    file: DiffFile::from_str(file_name)
                });
            }
            if !old_line_re.is_match(line) {
                line_number += 1;
            }
        }
        res
    }
}

fn extract_extension(file: &str) -> String {
    let re: Regex = Regex::new(r"\.(.*)").unwrap();
    match re.captures(file) {
        Some(cap) => cap.get(1).unwrap().as_str().to_owned(),
        None => "".to_owned()
    }
}

fn rem_first_letter(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
