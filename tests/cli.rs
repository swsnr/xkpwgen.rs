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
    assert!(result.status.success(),
            "xkpwgen failed with status {} and output:
stdout:
{}
stderr:
{}
",
            result.status,
            result.stdout,
            result.stderr);
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

fn assert_license_and_warranty(s: &str) {
    assert!(s.contains("GNU GPL version 3 or later"),
            "License name missing in output:
{}",
            s);
    assert!(s.contains("<http://gnu.org/licenses/gpl.html>"),
            "License URL missing in output:
{}",
            s);
    assert!(s.contains("There is NO WARRANTY, to the extent permitted by law."),
            "Warranty disclaimer missing in output:
{}",
            s);
}

#[test]
fn it_includes_license_and_warranty_in_help() {
    assert_license_and_warranty(&assert_run(&["--help"]).stdout);
}

#[test]
fn it_includes_license_and_warranty_in_version() {
    assert_license_and_warranty(&assert_run(&["--version"]).stdout);
}

#[test]
fn it_uses_only_words_from_the_wordlist() {
    let words = wordlist::builtin_words().into_iter().collect::<HashSet<_>>();
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
fn it_generates_different_passwords() {
    repeat_run!(result, &["-n", "100"], {
        let mut seen_passwords = HashSet::with_capacity(100);
        let mut duplicate_passwords = HashSet::new();
        for password in result.stdout.lines() {
            if !seen_passwords.insert(password) {
                duplicate_passwords.insert(password);
            }
        }

        assert!(duplicate_passwords.is_empty(),
                "Duplicate passwords found: {}",
                duplicate_passwords.into_iter().collect::<Vec<_>>().join(", "));

    });
}

#[test]
fn it_uses_the_original_eff_wordlist() {
    let stdout = assert_run(&["--words"]).stdout;
    assert_eq!(stdout.lines().collect::<Vec<_>>(),
               wordlist::builtin_words());
}
