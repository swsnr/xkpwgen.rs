// Copyright 2017 Sebastian Wiesner
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

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
