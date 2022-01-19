// Copyright 2017 Sebastian Wiesner <sebastian@swsnr.de>
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

//! Generate XKCD 936 passwords.
//!
//! ![](http://imgs.xkcd.com/comics/password_strength.png)
//!

#![deny(warnings, clippy::all)]

use clap::{AppSettings, FromArgMatches, IntoApp, StructOpt};
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

/// Words used by xkpwgen.
///
/// Combined all pokerware wordlists, from <https://github.com/skeeto/pokerware>.
///
/// Wordlists were released to the public domain.
///
/// See also <http://nullprogram.com/blog/2017/07/27/>.
const WORDS: &str = include_str!("words.txt");

/// Generate a single password from a wordlist.
///
/// Use the random generator `rng` to randomly draw from the wordlist `words` to generate a
/// password of the given `length`, and concatenate the resulting words with the `separator`.
fn generate_password<'a, R, W, T>(
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
    words
        .into_iter()
        .map(AsRef::as_ref)
        .choose_multiple(&mut rng, length)
        .join(separator)
}

static LICENSE: &str = "\
xkpwgen license Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>
There is NO WARRANTY, to the extent permitted by law.

wordlist by Christopher Wellons, released to public domain:
<https://github.com/skeeto/pokerware/tree/89a8fec541fdbe04fe15b5ad0d7986019240f741>
";

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(
        short = 'l',
        long = "length",
        default_value = "4",
        help = "The number of words per password"
    )]
    length_of_password: usize,
    #[structopt(
        short = 'n',
        long = "number",
        default_value = "10",
        help = "The number of passwords to generate"
    )]
    number_of_passwords: usize,
    #[structopt(
        short = 's',
        long = "separator",
        default_value = " ",
        help = "The separator between words in a password"
    )]
    word_separator: String,
}

#[test]
fn verify_options() {
    use clap::IntoApp;
    Options::into_app().debug_assert()
}

fn main() {
    let long_version = format!(
        "{}\n
{}",
        env!("CARGO_PKG_VERSION"),
        LICENSE
    );
    let matches = Options::into_app()
        .after_help(
            "\
xkpwgen copyright (C) 2017 Sebastian Wiesner <sebastian@swsnr.de>
wordlists copyright (C) 2017 Christopher Wellons",
        )
        .long_version(long_version.as_str())
        .mut_arg("version", |a| {
            a.help("Print version and license information")
        })
        .mut_arg("help", |a| a.help("Print this message"))
        .setting(AppSettings::DontCollapseArgsInUsage)
        .get_matches();
    let options = Options::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    let words: Vec<&'static str> = WORDS.lines().collect();
    for _ in 0..options.number_of_passwords {
        let password = generate_password(
            &mut thread_rng(),
            &words,
            options.length_of_password,
            &options.word_separator,
        );
        println!("{}", password);
    }
}
