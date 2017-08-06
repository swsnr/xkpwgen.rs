// Copyright 2017 Sebastian Wiesner
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Provide wordlists.
//!
//! This module contains the built-in list of words, and provides helpers to collect statistics
//! about a wordlist.

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
