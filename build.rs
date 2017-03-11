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

extern crate reqwest;
extern crate sha2;

use sha2::{Sha256, Digest};
use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::fs::File;

static URL: &'static str = "https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt";
static WORDLIST_SHA256: &'static str = "addd35536511597a02fa0a9ff1e5284677b8883b83e986e43f15a3db996b903e";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let eff_wordlist = Path::new(&out_dir).join("eff_wordlist.txt");

    let client = reqwest::Client::new().unwrap();

    let mut response = client.get(URL).header(reqwest::header::Connection::close()).send().unwrap();
    let mut wordlist_buffer = Vec::with_capacity(40000);
    response.read_to_end(&mut wordlist_buffer).unwrap();

    let mut hasher = Sha256::new();
    hasher.input(&wordlist_buffer);
    let hash = hasher.result().map(|b| format!("{:02x}", b)).join("");
    if hash != WORDLIST_SHA256 {
        panic!("SHA256 mismatch for EFF wordlist! Report issue to https://github.com/lunaryorn/xkpwgen.rs");
    }

    File::create(&eff_wordlist).unwrap().write_all(&wordlist_buffer).unwrap();
    println!("Wrote EFF wordlist to {}", eff_wordlist.to_string_lossy());
}
