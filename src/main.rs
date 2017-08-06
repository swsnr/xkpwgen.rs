// Copyright 2017 Sebastian Wiesner
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![deny(warnings)]

#[macro_use]
extern crate clap;
extern crate atty;
extern crate rand;
extern crate xkpwgen;

use clap::{AppSettings, Arg, ArgMatches};
use rand::os::OsRng;
use xkpwgen::generate_password;
use xkpwgen::wordlist::{WordlistStatistics, builtin_words};

static LICENSE: &'static str = "\
wordlist license CC BY 3.0 US: <http://creativecommons.org/licenses/by/3.0/us/>.

xkpwgen license either of
* Apache License, Version 2.0, <http://www.apache.org/licenses/LICENSE-2.0>
* MIT license, <http://opensource.org/licenses/MIT>
at your option.  There is NO WARRANTY, to the extent permitted by law.";

struct Options<'a> {
    print_wordlist: bool,
    length_of_password: usize,
    number_of_passwords: usize,
    word_separator: &'a str,
}

impl<'a> Options<'a> {
    fn from_matches(matches: &'a ArgMatches<'a>) -> clap::Result<Options<'a>> {
        let length = value_t!(matches.value_of("length"), usize)?;
        let number = value_t!(matches.value_of("number"), usize)?;
        // Separator has a default value, so we can safely unwrap here!
        let separator = matches.value_of("separator").unwrap();
        Ok(Options {
            print_wordlist: matches.is_present("words"),
            length_of_password: length,
            number_of_passwords: number,
            word_separator: separator,
        })
    }
}

fn main() {
    let words = builtin_words();
    let stats = WordlistStatistics::from_words(&words);
    let long_version = format!(
        "{}\n
EFF long wordlist July 2016: {} words (lengths: min {}, max {}, avg {:.2}, median: {})

{}",
        crate_version!(),
        stats.number_of_words,
        stats.min_word_length,
        stats.max_word_length,
        stats.avg_word_length,
        stats.med_word_length,
        LICENSE
    );
    let matches = app_from_crate!()
        .after_help(
            "\
xkpwgen  copyright (C) 2017 Sebastian Wiesner <swiesner@lunaryorn.com>
wordlist copyright (C) 2016 EFF <https://www.eff.org/copyright>",
        )
        .long_version(long_version.as_str())
        .version_message("Print version and license information")
        .help_message("Print this message")
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .default_value(" ")
                .help("The separator between words in a password"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .default_value("5")
                .help("The number of passwords to generate at once"),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .default_value("4")
                .help("The number of words in each password"),
        )
        .arg(Arg::with_name("words").long("words").help(
            "Print the internal wordlist and exit",
        ))
        .settings(
            &[
                AppSettings::DontCollapseArgsInUsage,
                // Don't put flags and options in separate --help groups
                AppSettings::UnifiedHelpMessage,
            ],
        )
        .get_matches();

    let options = Options::from_matches(&matches).unwrap_or_else(|e| e.exit());
    if options.print_wordlist {
        for word in builtin_words() {
            println!("{}", word);
        }
    } else {
        let mut rng = OsRng::new().expect("Failed to initialize random generator");
        for _ in 0..options.number_of_passwords {
            let password = generate_password(
                &mut rng,
                &words,
                options.length_of_password,
                options.word_separator,
            );
            println!("{}", password);
        }
    }

}
