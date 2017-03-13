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

#![deny(warnings)]

#[macro_use]
extern crate clap;
extern crate rand;

use clap::{App, AppSettings, Arg};

static EFF_WORDLIST: &'static str = include_str!(concat!(env!("OUT_DIR"), "/eff_wordlist.txt"));
static LICENSE: &'static str = {
    "License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law."
};

fn parse_diceware_list(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.split_whitespace().last().unwrap()).collect()
}

fn main() {
    let parse_result = App::new("xkpwgen")
        .version(crate_version!())
        .about("Generate XKCD 936 passwords")
        .after_help(format!("Copyright (C) 2017 Sebastian Wiesner <swiesner@lunaryorn.com>\n\n{}",
                            LICENSE)
                            .as_str())
        .version_message("Print version information")
        .help_message("Print this message")
        .arg(Arg::with_name("number")
                 .short("n")
                 .long("number")
                 .default_value("5")
                 .help("The number of passwords to generate at once"))
        .arg(Arg::with_name("length")
                 .short("l")
                 .long("length")
                 .default_value("4")
                 .help("The number of words in each password"))
        .settings(&[AppSettings::ColoredHelp,
                    AppSettings::DontCollapseArgsInUsage,
                    // Don't put flags and options in separate --help groups
                    AppSettings::UnifiedHelpMessage])
        .get_matches_safe();

    match parse_result {
        Ok(matches) => {
            let words = parse_diceware_list(EFF_WORDLIST);
            let password_length = value_t_or_exit!(matches.value_of("length"), usize);
            let number_of_passwords = value_t_or_exit!(matches.value_of("number"), usize);
            for _ in 0..number_of_passwords {
                println!("{}",
                         rand::sample(&mut rand::thread_rng(),
                                      words.iter().map(|s| *s),
                                      password_length)
                                 .join(" "));
            }
        }
        Err(error @ clap::Error { kind: clap::ErrorKind::VersionDisplayed, .. }) => {
            let mut words = parse_diceware_list(EFF_WORDLIST);
            words.sort_by_key(|w| w.chars().count());
            print!("\n
EFF long wordlist July 2017: {} words (min length {}, max length {})

{}",
                   words.len(),
                   words[0].chars().count(),
                   words.last()
                       .unwrap()
                       .chars()
                       .count(),
                   LICENSE);
            error.exit();
        }
        Err(error) => error.exit(),
    }
}
