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

fn phrases<'a>(s: &'a str, sep: &str) -> Vec<Vec<&'a str>> {
    s.lines().map(|w| w.split(sep).collect()).collect()
}

fn words<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
    s.lines().flat_map(|w| w.split(sep)).collect()
}

macro_rules! repeat_run {
    ($result:ident, $command:expr, $body:block) => {
        {
            for _ in 0..10 {
                let $result = assert_run($command);
                $body;
            }
        }
    };

    ($result:ident, $body:block) => {
        {
        let command: Vec<String> = Vec::new();
        repeat_run!($result, &command, $body);
        }
    };
}

#[test]
fn it_prints_a_single_phrase_by_default() {
    repeat_run!(result, {
        assert_eq!(phrases(&result.stdout, " ").len(), 1);
    })
}

#[test]
fn it_prints_four_words_per_phrase_by_default() {
    repeat_run!(result, {
        for words in phrases(&result.stdout, " ") {
            assert_eq!(words.len(), 4);
        }
    })
}

#[test]
fn it_prints_no_words_with_whitespace() {
    repeat_run!(result, {
        for word in words(&result.stdout, " ") {
            assert!(!word.contains(|c: char| c.is_whitespace()),
                    "Word {} contained whitespace!",
                    word);
        }
    })
}

#[test]
fn it_prints_no_empty_words() {
    repeat_run!(result, {
        for word in words(&result.stdout, " ") {
            assert!(word.len() > 0, "Got empty word");
        }
    })
}
