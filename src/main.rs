// Copyright 2017 Sebastian Wiesner
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Generate XKCD 936 passwords.
//!
//! ![](http://imgs.xkcd.com/comics/password_strength.png)
//!

#![deny(warnings)]

#[macro_use]
extern crate clap;
extern crate rand;

use clap::{AppSettings, Arg, ArgMatches};
use rand::{Rng, sample};
use rand::os::OsRng;

// Include the wordlist in the binary
static EFF_WORDLIST: &str = include_str!("eff_large_diceware_wordlist.txt");

/// Get the list of built-in words.
///
/// This function returns the [large diceware wordlist from the EFF][1] ([download][2]).  This list
/// contains 7776 common English words between 3 and 9 characters long, at an average of 7
/// characters per word.
///
/// The wordlist is freely available under the CC BY 3.0 US license; see
/// <https://www.eff.org/copyright> for more information about the copyright of the EFF.
///
/// [1]: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
/// [2]: https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt
pub fn builtin_words() -> Vec<&'static str> {
    EFF_WORDLIST.lines().collect()
}

/// Generate a single password from a wordlist.
///
/// Use the random generator `rng` to randomly draw from the wordlist `words` to generate a
/// password of the given `length`, and concatenate the resulting words with the `separator`.
pub fn generate_password<'a, R, W, T>(
    mut rng: &mut R,
    words: W,
    length: usize,
    separator: &str,
) -> String
where
    R: Rng,
    W: IntoIterator<Item = &'a T>,
    T: AsRef<str> + 'a,
{
    sample(&mut rng, words.into_iter().map(AsRef::as_ref), length).join(separator)
}


static LICENSE: &'static str = "\
wordlist license CC BY 3.0 US: <http://creativecommons.org/licenses/by/3.0/us/>.

xkpwgen license either of
* Apache License, Version 2.0, <http://www.apache.org/licenses/LICENSE-2.0>
* MIT license, <http://opensource.org/licenses/MIT>
at your option.  There is NO WARRANTY, to the extent permitted by law.";

struct Options<'a> {
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
            length_of_password: length,
            number_of_passwords: number,
            word_separator: separator,
        })
    }
}

fn main() {
    let words = builtin_words();
    let long_version = format!(
        "{}\n

{}",
        crate_version!(),
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
        .settings(
            &[
                AppSettings::DontCollapseArgsInUsage,
                // Don't put flags and options in separate --help groups
                AppSettings::UnifiedHelpMessage,
            ],
        )
        .get_matches();

    let options = Options::from_matches(&matches).unwrap_or_else(|e| e.exit());

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
