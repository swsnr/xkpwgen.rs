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
//! `generate_password` generates a single password from a wordlist.  A built-in wordlist is
//! provided by the `wordlist` module.

#![deny(warnings)]
#![deny(missing_docs)]

extern crate rand;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use rand::{Rng, sample};

pub mod wordlist;

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

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::TestResult;
    use rand::thread_rng;

    fn generate(length: usize, sep: &str) -> String {
        let words = wordlist::builtin_words();
        generate_password(&mut thread_rng(), &words, length, sep)
    }

    quickcheck! {
        fn has_expected_length(length: usize) -> TestResult {
            if length == 0 {
                TestResult::discard()
            } else {
                let password = generate(length, " ");
                TestResult::from_bool(password.split(' ').count() == length)
            }
        }

        fn uses_separator(length: usize, sep: String) -> TestResult {
            if length == 0 || sep.is_empty() || sep.chars().all(|c| c.is_alphanumeric()) {
                // Discard tests for empty passwords or separators, and for alphanumeric-only
                // separators because these might appear in the chosen words, making the separator
                // ambiguous and thus failing the test.
                TestResult::discard()
            } else {
                let password = generate(length, &sep);
                TestResult::from_bool(password.matches(&sep).count() == (length - 1))
            }
        }

        fn contains_only_words_from_wordlist(length: usize) -> TestResult {
            if length == 0 {
                TestResult::discard()
            } else {
                let words= wordlist::builtin_words();
                let password = generate_password(&mut thread_rng(), &words, length, " ");
                TestResult::from_bool(password.split(' ').all(|w| words.contains(&w)))
            }
        }

        fn repeated_passwords_are_different(length: usize) -> TestResult {
            if length == 0 {
                TestResult::discard()
            } else {
                let words = wordlist::builtin_words();
                let pw1 = generate_password(&mut thread_rng(), &words, length, " ");
                let pw2 = generate_password(&mut thread_rng(), &words, length, " ");
                TestResult::from_bool(pw1 != pw2)
            }
        }
    }
}
