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
use clap::{App, AppSettings, Arg, ArgMatches};
use rand::os::OsRng;
use xkpwgen::generate_password;
use xkpwgen::wordlist::builtin_words;

static LICENSE: &'static str = "\
wordlist license CC BY 3.0 US: <http://creativecommons.org/licenses/by/3.0/us/>.

xkpwgen license GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
xkpwgen is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.";

fn app() -> App<'static, 'static> {
    app_from_crate!()
        .after_help("\
xkpwgen  copyright (C) 2017 Sebastian Wiesner <swiesner@lunaryorn.com>
wordlist copyright (C) 2016 EFF <https://www.eff.org/copyright>")
        .version_message("Print version and license information")
        .help_message("Print this message")
        .arg(Arg::with_name("colour")
                 .alias("color")
                 .long("colour")
                 .possible_values(&["yes", "no", "auto"])
                 .default_value("auto")
                 .help("Whether to enable or disable coloured output."))
        .arg(Arg::with_name("separator")
                 .short("s")
                 .long("separator")
                 .default_value(" ")
                 .help("The separator between words in a password"))
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
        .arg(Arg::with_name("words")
                 .long("words")
                 .help("Print the internal wordlist and exit"))
        .settings(&[AppSettings::ColoredHelp,
                    AppSettings::DontCollapseArgsInUsage,
                    // Don't put flags and options in separate --help groups
                    AppSettings::UnifiedHelpMessage])
}

arg_enum! {
    enum YesNoAuto {
        Yes,
        No,
        Auto
    }
}

struct Options<'a> {
    print_wordlist: bool,
    length_of_password: usize,
    number_of_passwords: usize,
    colour_output: YesNoAuto,
    word_separator: &'a str,
}

impl<'a> Options<'a> {
    fn from_matches(matches: &'a ArgMatches<'a>) -> clap::Result<Options<'a>> {
        let length = value_t!(matches.value_of("length"), usize)?;
        let number = value_t!(matches.value_of("number"), usize)?;
        let colour = value_t!(matches, "colour", YesNoAuto)?;
        // Separator has a default value, so we can safely unwrap here!
        let separator = matches.value_of("separator").unwrap();
        Ok(Options {
               print_wordlist: matches.is_present("words"),
               length_of_password: length,
               number_of_passwords: number,
               colour_output: colour,
               word_separator: separator,
           })
    }

    fn colour_styles(&self) -> (Style, Style) {
        let enable_colours = match self.colour_output {
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
}

fn main() {
    let parse_result = app().get_matches_safe();

    match parse_result {
        Ok(matches) => {
            let options = Options::from_matches(&matches).unwrap_or_else(|e| e.exit());
            if options.print_wordlist {
                for word in builtin_words() {
                    println!("{}", word);
                }
            } else {
                let mut rng = OsRng::new().expect("Failed to initialize random generator");
                let words = builtin_words();
                let (even_style, odd_style) = options.colour_styles();
                for lineno in 0..options.number_of_passwords {
                    let style = if lineno % 2 == 0 {
                        even_style
                    } else {
                        odd_style
                    };
                    let password = generate_password(&mut rng,
                                                     &words,
                                                     options.length_of_password,
                                                     options.word_separator);
                    println!("{}", style.paint(password));
                }
            }
        }
        Err(error @ clap::Error { kind: clap::ErrorKind::VersionDisplayed, .. }) => {
            let mut words = builtin_words();
            words.sort_by_key(|w| w.chars().count());
            print!("\n
EFF long wordlist July 2016: {} words (min length {}, max length {})

{}",
                   words.len(),
                   words[0].chars().count(),
                   words.last().unwrap().chars().count(),
                   LICENSE);
            error.exit();
        }
        Err(error) => error.exit(),
    }
}
