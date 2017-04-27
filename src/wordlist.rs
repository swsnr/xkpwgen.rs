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

/// Provide statistics about a wordlist.
pub struct WordlistStatistics {
    /// The total number of words in the list.
    pub number_of_words: usize,
    /// The length of the shortest word.
    pub min_word_length: usize,
    /// The length of the longest word.
    pub max_word_length: usize,
    /// The average length of a word.
    pub avg_word_length: f64,
    /// The median of the length of all words.
    pub med_word_length: usize,
}

fn median(x: &[usize]) -> usize {
    let n = x.len();
    if n % 2 == 1 {
        x[(n - 1) / 2]
    } else {
        (x[n / 2] + x[(n / 2) - 1]) / 2
    }
}

impl WordlistStatistics {
    /// Collect statistics for the given wordlist.
    pub fn from_words<'a, W, T>(words: W) -> WordlistStatistics
        where W: IntoIterator<Item = &'a T>,
              T: AsRef<str> + 'a
    {
        let mut lengths: Vec<usize> = words
            .into_iter()
            .map(|w| w.as_ref().chars().count())
            .collect();
        if lengths.is_empty() {
            WordlistStatistics {
                number_of_words: 0,
                min_word_length: 0,
                max_word_length: 0,
                avg_word_length: 0.0,
                med_word_length: 0,
            }
        } else {
            lengths.sort();
            let sum_of_lengths: usize = lengths.iter().sum();
            WordlistStatistics {
                number_of_words: lengths.len(),
                min_word_length: lengths[0],
                max_word_length: lengths[lengths.len() - 1],
                avg_word_length: (sum_of_lengths as f64) / lengths.len() as f64,
                med_word_length: median(&lengths),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::builtin_words;
    use std::collections::HashSet;

    #[test]
    fn has_7776_words() {
        assert_eq!(builtin_words().len(), 7776);
    }

    #[test]
    fn contains_no_duplicate_words() {
        let mut seen_words = HashSet::with_capacity(8000);
        let mut duplicate_words = HashSet::new();
        for word in builtin_words() {
            if !seen_words.insert(word) {
                duplicate_words.insert(word);
            }
        }

        assert!(duplicate_words.is_empty(),
                "Duplicate words found: {}",
                duplicate_words
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(" "));
    }

    #[test]
    fn contains_no_empty_words() {
        for word in builtin_words() {
            assert!(!word.is_empty(), "Got empty word");
        }
    }

    #[test]
    fn contains_no_words_with_whitespace() {
        for word in builtin_words() {
            assert!(!word.contains(|c: char| c.is_whitespace()),
                    "Word {} contained whitespace!",
                    word);
        }
    }
}
