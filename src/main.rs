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

#[macro_use]
extern crate clap;
extern crate rand;

use clap::{App, Arg};

static EFF_WORDLIST: &'static str = include_str!(concat!(env!("OUT_DIR"), "/eff_wordlist.txt"));

fn parse_diceware_list(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.split_whitespace().last().unwrap()).collect()
}

fn main() {
    let matches = App::new("xkpwgen")
        .version(&crate_version!())
        .about("Generate XKCD 936 passwords")
        .after_help("Copyright (C) 2017 Sebastian Wiesner <swiesner@lunaryorn.com>
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.")
        .get_matches();

    let words = parse_diceware_list(EFF_WORDLIST);
    println!("{}",
             rand::sample(&mut rand::thread_rng(), words, 4).join(" "));
}
