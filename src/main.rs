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

#[macro_use]
extern crate clap;
extern crate atty;
extern crate rand;
extern crate ansi_term;
extern crate xkpwgen;

use ansi_term::Colour;
use ansi_term::Style;
use clap::{App, AppSettings, Arg};
use rand::os::OsRng;
use xkpwgen::generate_password;
use xkpwgen::wordlist::builtin_words;

macro_rules! license {
    () => {
    "License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law."
    }
}


arg_enum! {
    enum YesNoAuto {
        Yes,
        No,
        Auto
    }
}

fn alternating_styles(colour_setting: &YesNoAuto) -> (Style, Style) {
    let enable_colours = match *colour_setting {
        YesNoAuto::Auto => atty::is(atty::Stream::Stdout),
        YesNoAuto::Yes => true,
        YesNoAuto::No => false,
    };
    if enable_colours {
        (Style::new().fg(Colour::Cyan), Style::new().fg(Colour::Purple))
    } else {
        (Style::new(), Style::new())
    }
}

fn app() -> App<'static, 'static> {
    app_from_crate!()
        .after_help(concat!("Copyright (C) 2017 Sebastian Wiesner <swiesner@lunaryorn.com>\n\n",
                            license!()))
        .version_message("Print version information")
        .help_message("Print this message")
        .arg(Arg::with_name("colour")
                 .alias("color")
                 .long("colour")
                 .possible_values(&["yes", "no", "auto"])
                 .default_value("auto")
                 .help("Whether to enable or disable coloured output."))
        .arg(Arg::with_name("number")
                 .short("n")
                 .long("number")
                 .default_value("5")
                 .help("The number of passwords to generate at once"))
        .arg(Arg::with_name("length")
                 .short("l")
                 .long("length")
                 .default_value("4")
                 .help("The number of words in each password"))
        .arg(Arg::with_name("words").long("words").help("Print the internal wordlist and exit"))
        .settings(&[AppSettings::ColoredHelp,
                    AppSettings::DontCollapseArgsInUsage,
                    // Don't put flags and options in separate --help groups
                    AppSettings::UnifiedHelpMessage])
}

fn main() {
    let parse_result = app().get_matches_safe();

    match parse_result {
        Ok(matches) => {
            if matches.is_present("words") {
                for word in builtin_words() {
                    println!("{}", word);
                }
            } else {
                let mut rng = OsRng::new().expect("Failed to initialize random generator");
                let words = builtin_words();
                let password_length = value_t_or_exit!(matches.value_of("length"), usize);
                let number_of_passwords = value_t_or_exit!(matches.value_of("number"), usize);
                let (even, odd) =
                    alternating_styles(&value_t_or_exit!(matches, "colour", YesNoAuto));
                for lineno in 0..number_of_passwords {
                    let style = if lineno % 2 == 0 { even } else { odd };
                    let password = generate_password(&mut rng, &words, password_length, " ");
                    println!("{}", style.paint(password));
                }
            }
        }
        Err(error @ clap::Error { kind: clap::ErrorKind::VersionDisplayed, .. }) => {
            let mut words = builtin_words();
            words.sort_by_key(|w| w.chars().count());
            print!("\n
EFF long wordlist July 2017: {} words (min length {}, max length {})

{}",
                   words.len(),
                   words[0].chars().count(),
                   words.last()
                       .unwrap()
                       .chars()
                       .count(),
                   license!());
            error.exit();
        }
        Err(error) => error.exit(),
    }
}
