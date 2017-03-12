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
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// We use the diceware list of the EFF, which contains 7776 common English words between three and
// nine characters.  I'm unsure about the licensing situation concerning this list so I'm not
// shipping it with the sources but instead download it during the build and use its hash to verify
// its authenticity.
static URL: &'static str = "https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt";
static WORDLIST_SHA256: &'static [u8] = &[0xad, 0xdd, 0x35, 0x53, 0x65, 0x11, 0x59, 0x7a, 0x02,
                                          0xfa, 0x0a, 0x9f, 0xf1, 0xe5, 0x28, 0x46, 0x77, 0xb8,
                                          0x88, 0x3b, 0x83, 0xe9, 0x86, 0xe4, 0x3f, 0x15, 0xa3,
                                          0xdb, 0x99, 0x6b, 0x90, 0x3e];

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let eff_wordlist = Path::new(&out_dir).join("eff_wordlist.txt");

    let client = reqwest::Client::new().unwrap();

    let mut response = client.get(URL)
        .header(reqwest::header::Connection::close())
        .send()
        .unwrap();
    let mut wordlist_buffer = Vec::with_capacity(40000);
    response.read_to_end(&mut wordlist_buffer).unwrap();

    let mut hasher = Sha256::new();
    hasher.input(&wordlist_buffer);
    let hash = hasher.result();
    if *hash != *WORDLIST_SHA256 {
        panic!("SHA256 mismatch for EFF wordlist! Report issue to \
                https://github.com/lunaryorn/xkpwgen.rs");
    }

    File::create(&eff_wordlist).unwrap().write_all(&wordlist_buffer).unwrap();
    println!("Wrote EFF wordlist to {}", eff_wordlist.to_string_lossy());
}
