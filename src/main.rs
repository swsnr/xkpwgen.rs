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

#![deny(warnings, clippy::all)]

use clap::Parser;
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

fn long_version() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "\n",
        "\
xkpwgen license Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>
There is NO WARRANTY, to the extent permitted by law.

wordlist by Christopher Wellons, released to public domain:
<https://github.com/skeeto/pokerware/tree/89a8fec541fdbe04fe15b5ad0d7986019240f741>"
    )
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_version = long_version())]
struct Options {
    /// Number of words per password
    #[arg(short, long, default_value_t = 4)]
    length: usize,
    #[arg(short, long, default_value_t = 10)]
    /// Number of passwords to generate
    number: usize,
    /// separator between words in a password
    #[arg(short, long, default_value = " ")]
    separator: String,
}

fn main() {
    let options = Options::parse();
    let words: Vec<&'static str> = WORDS.lines().collect();
    for _ in 0..options.number {
        let password = generate_password(
            &mut thread_rng(),
            &words,
            options.length,
            &options.separator,
        );
        println!("{}", password);
    }
}
