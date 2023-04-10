use regex::{Regex, RegexBuilder, RegexSet};
use std::{
    process::{
        Command, Output
    },
    fs,
    vec
};

const DEFAULT_REGEX: &[&str; 6] = &[
    r"//",
    r"/\*",
    r"\*/",
    r"error_log",
    r"console.log",
    r"cagada",
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

#[derive(Debug, PartialEq)]
pub struct DiffFile {
    pub name: String,
    pub extension: Option<String>,
    pub status: DiffFileStatus,
}

#[derive(Debug, Default, Clone)]
pub struct Cagada {
    pub line_number: usize,
    pub line: String,
}

#[derive(Debug, PartialEq, Default)]
pub enum DiffFileStatus {
    New,
    Deleted,
    Modified,
    #[default]
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
    #[allow(dead_code)]
    pub fn from_string(text: String) -> Self {
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
            extension: DiffFile::extract_extension(filename),
            status: DiffFileStatus::Undefined,
        }
    }

    fn extract_extension(file: &str) -> Option<String> {
        let re: Regex = Regex::new(r"\.(.*)").unwrap();
        re.captures(file).map(|cap| cap.get(1).unwrap().as_str().to_owned())
    }
}

impl Cagada {
    fn default_regex() -> RegexSet {
        RegexSet::new(DEFAULT_REGEX).unwrap()
    }

    fn get(source: IterableText) -> Option<Vec<Cagada>> {
        let mut res: Vec<Cagada> = vec![];
        let re: RegexSet = Cagada::default_regex();

        for (index, line) in source.lines.into_iter().enumerate() {
            if re.is_match(&line) {
                res.push(Cagada {
                    line: line.to_owned(),
                    line_number: source.line_numbers[index],
                });
            }
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
        let vec = lines.split('\n').collect::<Vec<&str>>();

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
        IterableText { lines, line_numbers }
    }

    fn rem_first_letter(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next();
        chars.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterable_text_from_string() {
        let text = "test\ntest2\ntest3".to_owned();
        let iterable = IterableText::from_string(text);
        assert_eq!(iterable.lines.len(), 3);
        assert_eq!(iterable.line_numbers.len(), 3);
    }

    #[test]
    fn test_iterable_text_from_file() {
        let file_path = "tests/diff-sample";
        let iterable = IterableText::from_file(file_path);
        assert_eq!(iterable.lines.len(), 224);
        assert_eq!("-use clap::{Parser, arg, /* Command */};", iterable.lines[136]);
    }

    #[test]
    fn test_diff_file_from_str_with_extension() {
        let filename = "test.txt";
        let file = DiffFile::from_str(filename);
        assert_eq!(file.name, filename);
        assert_eq!(file.extension, Some("txt".to_owned()));
        assert_eq!(file.status, DiffFileStatus::Undefined);
    }

    #[test]
    fn test_diff_full() {
        let file = DiffFile::from_str("tests/full-sample");
        let diff = Diff::full(file);
        for (index, cagada) in diff.cagadas.as_ref().unwrap().into_iter().enumerate() {
            if index == 3 {
                println!("{}", cagada.line);
                assert_eq!(diff.cagada_count, 8);
                assert_eq!(cagada.line, "    #[arg(long)error_log]".to_owned());
            }
        }
    }

    #[test]
    fn test_diff_default() {
        let file = DiffFile::from_str("test.txt");
        let diff = Diff::default(file);
        assert_eq!(diff.cagada_count, 0);
    }

    #[test]
    fn test_by_hash1() {
        let mut com = crate::Cli::git_show_command();
        com.arg("53cafd8665e86564a4fa1297c8c12e4c028ddcd1");
        let mut files = DiffFile::get(&mut com).unwrap();
        let file = files.remove(1);
        let diff = Diff::new(file, com);

        assert_eq!(diff.cagada_count, 16);
        assert_eq!(
            diff.cagadas.as_ref().unwrap().get(2).unwrap().line,
            "    pub fn get () /* -> Self */ {".to_owned());
        assert_eq!(diff.cagadas.as_ref().unwrap().get(2).unwrap().line_number, 14);
    }

    #[test]
    fn test_by_hash2() {
        let mut com = crate::Cli::git_show_command();
        com.arg("53cafd8665e86564a4fa1297c8c12e4c028ddcd1");
        let file = DiffFile::from_str("src/git.rs");
        let diff = Diff::new(file, com);

        assert_eq!(diff.cagada_count, 16);
        assert_eq!(
            diff.cagadas.as_ref().unwrap().get(2).unwrap().line,
            "    pub fn get () /* -> Self */ {".to_owned());
        assert_eq!(diff.cagadas.as_ref().unwrap().get(2).unwrap().line_number, 14);
    }

    #[test]
    fn test_diff_file_vs_from_str() {
        let mut com = crate::Cli::git_show_command();
        com.arg("53cafd8665e86564a4fa1297c8c12e4c028ddcd1");

        let mut files = DiffFile::get(&mut com).unwrap();
        let file1 = files.remove(1);

        let file2 = DiffFile::from_str("src/git.rs");

        assert_eq!(file1, file2);
    }
}
