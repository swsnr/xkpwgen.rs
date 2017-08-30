//    Copyright 2017 Sebastian Wiesner <swiesner@lunaryorn.com>
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

//! Generate XKCD 936 passwords.
//!
//! ![](http://imgs.xkcd.com/comics/password_strength.png)
//!

#![deny(warnings)]

#[macro_use]
extern crate clap;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use rand::{Rng, sample, thread_rng};


/// Words to generate passwords from.
///
/// We use the [pokerware words][1] from  by Christopher Wellons which he released
/// to public domain.
///
/// His blog post [Introducing the Pokerware Secure Passphrase Generator][2] explains where he
/// obtained these word lists from.
///
/// [1]: https://github.com/skeeto/pokerware
/// [2]: http://nullprogram.com/blog/2017/07/27/
mod words {
    lazy_static! {
        /// Formal words.
        ///
        /// > The “formal” list is derived in part from Google’s Ngram Viewer, with my own
        /// > additional filters and tweaking. It’s called “formal” because the ngrams come from
        /// > formal publications and represent more formal kinds of speech.
        ///
        /// Source: <http://nullprogram.com/blog/2017/07/27/>
        pub static ref FORMAL: Vec<&'static str> = include_str!("words/formal.txt")
            .lines()
            .collect();

        /// Slang words.
        ///
        /// > The “slang” list is derived from every reddit comment between December 2005 and May
        /// > 2017, tamed by the same additional filters. I have this data on hand, so I may as well
        /// > put it to use. I figured more casually-used words would be easier to remember. Due to
        /// > my extra filtering, there’s actually a lot of overlap between these lists, so the
        /// > differences aren’t too significant.
        ///
        /// Source: <http://nullprogram.com/blog/2017/07/27/>.
        ///
        /// See [A Showerthoughts Fortune File][1] for the source of Reddit comments.
        ///
        /// [1]: http://nullprogram.com/blog/2016/12/01/
        pub static ref SLANG: Vec<&'static str> = include_str!("words/slang.txt")
            .lines()
            .collect();
    }
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

/// CLI arguments to this tool.
mod args {
    use super::words;
    use clap::{Result, AppSettings, Arg, ArgMatches};

    static LICENSE: &'static str = "\
wordlist license CC BY 3.0 US: <http://creativecommons.org/licenses/by/3.0/us/>.

xkpwgen license Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

There is NO WARRANTY, to the extent permitted by law.";

    arg_enum! {
        /// Which list of words to use.
        #[derive(Clone, Copy, Debug)]
        pub enum ListOfWords {
            Slang,
            Formal
        }
    }

    pub struct Options {
        pub length_of_password: usize,
        pub number_of_passwords: usize,
        pub word_separator: String,
        pub list_of_words: ListOfWords,
    }

    impl Options {
        pub fn parse() -> Result<Options> {
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
                .arg(
                    Arg::with_name("list_of_words")
                        .short("w")
                        .long("--words")
                        .possible_values(&ListOfWords::variants())
                        .default_value(ListOfWords::variants()[0])
                        .help("The list of words to use to generate a password"),
                )
                .settings(
                    &[
                        AppSettings::DontCollapseArgsInUsage,
                        // Don't put flags and options in separate --help groups
                        AppSettings::UnifiedHelpMessage,
                    ],
                )
                .get_matches();

            Options::from_matches(&matches)
        }

        /// Extract our options from matched CLI arguments.
        fn from_matches<'a>(matches: &'a ArgMatches<'a>) -> Result<Options> {
            let length = value_t!(matches.value_of("length"), usize)?;
            let number = value_t!(matches.value_of("number"), usize)?;
            let list_of_words = value_t!(matches.value_of("list_of_words"), ListOfWords)?;
            let separator = value_t!(matches.value_of("separator"), String)?;
            Ok(Options {
                length_of_password: length,
                number_of_passwords: number,
                word_separator: separator,
                list_of_words: list_of_words,
            })
        }

        /// Get the words selected by CLI options.
        pub fn words<'a>(&self) -> &'a Vec<&'static str> {
            match self.list_of_words {
                ListOfWords::Slang => &*words::SLANG,
                ListOfWords::Formal => &*words::FORMAL,
            }
        }
    }
}



fn main() {
    use args::Options;

    let options = Options::parse().unwrap_or_else(|e| e.exit());

    for _ in 0..options.number_of_passwords {
        let password = generate_password(
            &mut thread_rng(),
            options.words(),
            options.length_of_password,
            &options.word_separator,
        );
        println!("{}", password);
    }
}
