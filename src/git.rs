use regex::{Regex, RegexBuilder, RegexSet};
use std::{
    process::{
        Command, Output
    },
    fs,
    vec
};

const DEFAULT_REGEX: &'static [&str] = &[
    r"^\s*//",
    r"^\s*/\*",
    r"^\s*\*/",
    r"^\s*error_log",
    r"^\s*console.log",
    r"^\s*cagada",
];

const GIT_LINE_NUMBER_REGEX: &str = r"@@ -\d+,\d+ \+(\d+)";
const GIT_NEW_LINE_REGEX: &str = r"^\+[^\+].*";
const GIT_REMOVED_LINE_REGEX: &str = r"^\-[^\-]*?";

#[derive(Debug)]
pub struct Diff {
    pub file: DiffFile,
    pub cagadas: Option<Vec<Cagada>>,
    pub cagada_count: usize,
}

#[derive(Debug)]
pub struct DiffFile {
    pub name: String,
    pub extension: Option<String>,
    pub status: DiffFileStatus,
}

#[derive(Debug, Default)]
pub struct Cagada {
    pub line_number: usize,
    pub line: String,
}

#[derive(Debug)]
pub enum DiffFileStatus {
    New,
    Deleted,
    Modified,
    Undefined,
}

struct Git {
    command: Command,
}

#[derive(Debug)]
pub struct IterableText {
    lines: Vec<String>,
    line_numbers: Vec<usize>,
}

impl IterableText {
    pub fn _from_string(text: String) -> Self {
        let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
        let line_numbers = (1..=lines.len()).collect::<Vec<usize>>();
        IterableText { lines, line_numbers }
    }

    pub fn from_file(filename: &str) -> Self {
        let f = fs::read_to_string(filename).unwrap();
        let lines: Vec<String> = f.lines()
            .map(|line| line.to_string())
            .collect();

        let line_numbers = (1..=lines.len()).collect::<Vec<usize>>();

        IterableText { lines, line_numbers }
    }
}

impl Diff {
    pub fn new (file: DiffFile, command: Command) -> Self {
        let mut git_parser = Git { command };
        let source = git_parser.parse_command(&file.name);

        let cagadas = Cagada::get(source);
        let cagada_count = cagadas.iter().flatten().count();

        if cagada_count == 0 {
            return Diff::default(file);
        }

        Diff { file, cagadas, cagada_count }
    }

    pub fn full (file: DiffFile) -> Self {
        let filename = &file.name;
        let source = IterableText::from_file(filename);

        let cagadas = Cagada::get(source);
        let cagada_count = cagadas.iter().flatten().count();

        if cagada_count == 0 {
            return Diff::default(file);
        }

        Diff { file, cagadas, cagada_count }
    }

    pub fn default(file: DiffFile) -> Self {
        Diff { file, cagadas: None, cagada_count: 0usize }
    }
}

impl DiffFile {
    pub fn get(command: &mut Command) -> Option<Vec<DiffFile>> {
        let re: Regex = RegexBuilder::new(
            r"^:(?:[[:alnum:]]+\s){4}([MDA])\s+(.*)$"
        )
            .multi_line(true)
            .build()
            .unwrap();

        command.arg("--raw");

        let output: Output = command.output().unwrap();
        let text: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            _ => return None,
        };

        let mut res: Vec<DiffFile> = vec![];

        for capture in re.captures_iter(&text) {
            let name = capture.get(2).unwrap().as_str().to_owned();
            let status: DiffFileStatus = match capture.get(1).unwrap().as_str() {
                "A" => DiffFileStatus::New,
                "D" => DiffFileStatus::Deleted,
                "M" => DiffFileStatus::Modified,
                _ => DiffFileStatus::Undefined,
            };
            let extension = DiffFile::extract_extension(&name);

            res.push(DiffFile {
                name,
                status,
                extension
            })
        }
        Some(res)
    }

    pub fn from_str(filename: &str) -> Self {
        DiffFile {
            name: filename.to_owned(),
            extension: DiffFile::extract_extension(&filename),
            status: DiffFileStatus::Undefined,
        }
    }

    fn extract_extension(file: &str) -> Option<String> {
        let re: Regex = Regex::new(r"\.(.*)").unwrap();
        match re.captures(file) {
            Some(cap) => Some(cap.get(1).unwrap().as_str().to_owned()),
            None => None
        }
    }
}

impl Cagada {
    fn default_regex() -> RegexSet {
        RegexSet::new(DEFAULT_REGEX).unwrap()
    }

    fn get(source: IterableText) -> Option<Vec<Cagada>> {
        let mut index: usize = 0usize;
        let mut res: Vec<Cagada> = vec![];
        let re: RegexSet = Cagada::default_regex();

        for line in source.lines {
            if re.is_match(&line) {
                res.push(Cagada {
                    line: line.to_owned(),
                    line_number: source.line_numbers[index],
                });
            }
            index += 1;
        }

        if res.is_empty() {
            return None;
        }
        Some(res)
    }

    fn _set_capture_pattern() -> RegexSet {
        todo!();
    }
}

impl Git {
    fn parse_command(&mut self, filename: &str) -> IterableText {
        self.command.arg(filename);
        let output: Output = self.command.output().unwrap();
        let lines = String::from_utf8(output.stdout).unwrap_or_default();
        let vec = lines.split("\n").collect::<Vec<&str>>();

        let line_re: Regex = Regex::new(GIT_LINE_NUMBER_REGEX).unwrap();
        let new_line_re: Regex = Regex::new(GIT_NEW_LINE_REGEX).unwrap();
        let old_line_re: Regex = Regex::new(GIT_REMOVED_LINE_REGEX).unwrap();

        let mut line_number: usize = 0;
        let mut lines: Vec<String> = vec![];
        let mut line_numbers: Vec<usize> = vec![];

        for line in vec {
            if line_re.is_match(line) {
                line_number = line_re.captures(line).unwrap()
                    .get(1).unwrap()
                    .as_str().parse::<usize>().unwrap() - 1;
                }

            if new_line_re.is_match(line) {
                lines.push(Git::rem_first_letter(line).to_owned());
                line_numbers.push(line_number);
            }

            if !old_line_re.is_match(line) {
                line_number += 1;
            }
        }
    let res = IterableText { lines, line_numbers };

    res

    }

    fn rem_first_letter(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next();
        chars.as_str()
    }
}
