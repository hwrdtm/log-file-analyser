#![allow(dead_code)]

use core::fmt;
use std::fs;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: String,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            inner: e.to_string(),
        }
    }
}

pub struct MatchedLine {
    line_number: usize,
    line: String,
}

pub struct MatchResult {
    matched_lines: Vec<MatchedLine>,
}

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Loop over each of the matched lines and write them to the formatter.
        for matched_line in &self.matched_lines {
            writeln!(f, "{}: {}", matched_line.line_number, matched_line.line)?;
        }

        Ok(())
    }
}

pub fn match_strings<T>(strings_to_match: Vec<T>, file_path: T) -> Result<MatchResult>
where
    T: AsRef<str>,
{
    // First read the file.
    let file_contents = fs::read_to_string(file_path.as_ref())
        .map_err(|e| Error::from(e))?;

    // Then, iterate over each line and collect the matched lines as well as the line number.
    let mut matched_lines = Vec::new();
    for (line_number, line) in file_contents.lines().enumerate() {
        for string_to_match in &strings_to_match {
            if line.contains(string_to_match.as_ref()) {
                matched_lines.push(MatchedLine {
                    line_number: line_number + 1,
                    line: line.to_string(),
                });
            }
        }
    }

    Ok(MatchResult { matched_lines })
}