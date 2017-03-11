/*
 * Copyright (C) 2017  Sebastian Wiesner <swiesner@lunaryorn.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::ffi::OsStr;
use std::process::{Command, ExitStatus, Output};

struct Result {
    stdout: String,
    stderr: String,
    status: ExitStatus,
}

impl From<Output> for Result {
    fn from(output: Output) -> Result {
        Result {
            status: output.status,
            stdout: String::from_utf8(output.stdout).expect("Failed to decode stdout"),
            stderr: String::from_utf8(output.stderr).expect("Failed to decode stderr"),
        }
    }
}

fn run<S: AsRef<OsStr>>(args: &[S]) -> Result {
    Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to run xkpwgen for testing")
        .into()
}

fn assert_run<S: AsRef<OsStr>>(args: &[S]) -> Result {
    let result = run(args);
    assert!(result.status.success(),
            "xkpwgen failed with output:
stdout:
{}
stderr:
{}
",
            result.stdout,
            result.stderr);
    result
}

fn parse_phrases<'a>(s: &'a str, sep: &str) -> Vec<Vec<&'a str>> {
    s.lines().map(|w| w.split(sep).collect()).collect()
}

#[test]
fn it_prints_a_single_phrase_by_default() {
    let result = assert_run::<String>(&[]);
    let phrases = parse_phrases(&result.stdout, " ");
    assert!(phrases.len() == 1,
            "Expected one phrase, got {}",
            phrases.len());
}

#[test]
fn it_prints_four_words_per_phrase_by_default() {
    for _ in 0..10 {
        let result = assert_run::<String>(&[]);
        let phrases = parse_phrases(&result.stdout, " ");
        for words in phrases {
            assert!(words.len() == 4, "Expected 4 words, got {}", words.len());
        }
    }
}

#[test]
fn it_prints_no_words_with_whitespace() {
    for _ in 0..10 {
        let result = assert_run::<String>(&[]);
        for word in parse_phrases(&result.stdout, " ").iter().flat_map(|ws| ws.iter()) {
            assert!(!word.contains(|c: char| c.is_whitespace()),
                    "Word {} contained whitespace!",
                    word);
        }
    }
}

#[test]
fn it_prints_no_empty_words() {
    for _ in 0..10 {
        let result = assert_run::<String>(&[]);
        for word in parse_phrases(&result.stdout, " ").iter().flat_map(|ws| ws.iter()) {
            assert!(word.len() > 0, "Got empty word");
        }
    }
}
