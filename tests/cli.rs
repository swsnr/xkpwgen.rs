// Copyright 2017 Sebastian Wiesner
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate xkpwgen;

use std::collections::HashSet;
use std::ffi::OsStr;
use std::process::{Command, ExitStatus, Output};
use xkpwgen::wordlist;

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
    assert!(
        result.status.success(),
        "xkpwgen failed with status {} and output:
stdout:
{}
stderr:
{}
",
        result.status,
        result.stdout,
        result.stderr
    );
    result
}

fn all_words<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
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
fn it_includes_eff_copyright_in_help() {
    let s = assert_run(&["--help"]).stdout;
    assert!(
        s.contains(
            "wordlist copyright (C) 2016 EFF <https://www.eff.org/copyright>",
        ),
        "EFF wordlist copyright missing from --help:
{}",
        s
    );
}

#[test]
fn it_uses_only_words_from_the_wordlist() {
    let words = wordlist::builtin_words()
        .into_iter()
        .collect::<HashSet<_>>();
    repeat_run!(result, {
        for word in all_words(&result.stdout, " ") {
            assert!(words.contains(word), "Word {} not in EFF wordlist!", word);
        }
    });
}

#[test]
fn it_generates_five_phrases_by_default() {
    repeat_run!(result, {
        assert_eq!(result.stdout.lines().count(), 5);
    })
}

#[test]
fn it_generates_the_given_number_of_passwords() {
    repeat_run!(result, &["-n", "10"], {
        assert_eq!(result.stdout.lines().count(), 10);
    })
}

#[test]
fn it_generates_four_words_per_password_by_default() {
    repeat_run!(result, {
        for password in result.stdout.lines() {
            assert_eq!(all_words(password, " ").len(), 4);
        }
    })
}

#[test]
fn it_generates_the_given_number_of_words_per_password() {
    repeat_run!(result, &["-l", "9"], {
        for password in result.stdout.lines() {
            assert_eq!(all_words(password, " ").len(), 9)
        }
    });
}

#[test]
fn it_uses_the_given_separator() {
    repeat_run!(result, &["-s", ":"], {
        for password in result.stdout.lines() {
            assert_eq!(all_words(password, ":").len(), 4);
        }
    });
}

#[test]
fn it_generates_different_passwords() {
    repeat_run!(result, &["-n", "100"], {
        let mut seen_passwords = HashSet::with_capacity(100);
        let mut duplicate_passwords = HashSet::new();
        for password in result.stdout.lines() {
            if !seen_passwords.insert(password) {
                duplicate_passwords.insert(password);
            }
        }

        assert!(
            duplicate_passwords.is_empty(),
            "Duplicate passwords found: {}",
            duplicate_passwords.into_iter().collect::<Vec<_>>().join(
                ", ",
            )
        );

    });
}

#[test]
fn it_uses_the_original_eff_wordlist() {
    let stdout = assert_run(&["--words"]).stdout;
    assert_eq!(
        stdout.lines().collect::<Vec<_>>(),
        wordlist::builtin_words()
    );
}
