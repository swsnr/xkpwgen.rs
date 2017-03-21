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

#![feature(test)]

extern crate test;
extern crate rand;
extern crate xkpwgen;

use rand::os::OsRng;
use rand::thread_rng;
use test::Bencher;
use xkpwgen::*;

const SHORT: usize = 4;
const LONG: usize = 100;

#[bench]
fn short_password_with_thread_rng(b: &mut Bencher) {
    let words = wordlist::builtin_words();
    let mut rng = thread_rng();
    b.iter(|| generate_password(&mut rng, &words, SHORT, " "));
}

#[bench]
fn long_password_with_thread_rng(b: &mut Bencher) {
    let words = wordlist::builtin_words();
    let mut rng = thread_rng();
    b.iter(|| generate_password(&mut rng, &words, LONG, " "));
}

#[bench]
fn short_password_with_os_rng(b: &mut Bencher) {
    let words = wordlist::builtin_words();
    let mut rng = OsRng::new().expect("Failed to create OS random generator");
    b.iter(|| generate_password(&mut rng, &words, SHORT, " "));
}

#[bench]
fn long_password_with_os_rng(b: &mut Bencher) {
    let words = wordlist::builtin_words();
    let mut rng = OsRng::new().expect("Failed to create OS random generator");
    b.iter(|| generate_password(&mut rng, &words, LONG, " "));
}
