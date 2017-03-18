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

static EFF_WORDLIST: &'static str = include_str!(concat!(env!("OUT_DIR"), "/eff_wordlist.txt"));

pub fn builtin_words() -> Vec<&'static str> {
    EFF_WORDLIST.lines().collect()
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
        for word in builtin_words().into_iter() {
            if !seen_words.insert(word) {
                duplicate_words.insert(word);
            }
        }

        assert!(duplicate_words.is_empty(),
                "Duplicate words found: {}",
                duplicate_words.into_iter().collect::<Vec<_>>().join(" "));
    }

    #[test]
    fn contains_no_empty_words() {
        for word in builtin_words() {
            assert!(word.len() > 0, "Got empty word");
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
