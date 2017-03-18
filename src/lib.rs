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

extern crate rand;

use rand::{Rng, sample};

pub mod wordlist;

pub fn generate_password<'a, R, W, T>(mut rng: &mut R,
                                      words: W,
                                      length: usize,
                                      separator: &str)
                                      -> String
    where R: Rng,
          W: IntoIterator<Item = &'a T>,
          T: AsRef<str> + 'a
{
    sample(&mut rng, words.into_iter().map(AsRef::as_ref), length).join(separator)
}
