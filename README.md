# xkpwgen [![Travis branch][travis-badge]][travis]

[travis-badge]: https://img.shields.io/travis/lunaryorn/xkpwgen.rs/master.svg?maxAge=2592000
[travis]: https://travis-ci.org/lunaryorn/xkpwgen.rs

Generate [XKCD 936](https://xkcd.com/936/) passwords:

![](http://imgs.xkcd.com/comics/password_strength.png)

## Install

```console
$ cargo install xkpwgen
```

Building this tool requires network access because the build script fetches the
wordlist from the EFF (see below).

## Use

Invoke `xkpwgen` to generate five passwords:

```console
$ xkpwgen
veneering hazing aging diner
hypertext repose salvaging barometer
quote parchment boxy hurt
gift womanhood step mammary
october hemstitch darkness agony
```

Use `-l` to change the length of passwords, and `-n` to change the number of
passwords:

```console
$ xkpwgen -l 10 -n 2
paralyses unawake both wish gawk islamist taco charity shorts barmaid
submersed tackle font gentile navy same boxing tartness trodden confined
```

`xkpwgen --words` prints the list of words used in passwords and exists.

See `xkpwgen --help` for more information.

## Wordlist

This tool uses the [EFF long wordlist][1] which contains 7776 common English
words between three and nine characters in length, at an average of seven
characters per word.

Due to unclear licensing of the wordlist is not contained in this repository.
The build script downloads and verifies the wordlist from the above URL before
embedding it in the final executable.  For this reason, the license of the final
executable is unclear as well, distribute at your own risk.

If you happen to have licensing information for the wordlist, please let me
know.

[1]: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases

## Copyright

> Copyright (C) 2017  Sebastian Wiesner
>
> This program is free software: you can redistribute it and/or modify
> it under the terms of the GNU General Public License as published by
> the Free Software Foundation, either version 3 of the License, or
> (at your option) any later version.
>
> This program is distributed in the hope that it will be useful,
> but WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
> GNU General Public License for more details.
>
> You should have received a copy of the GNU General Public License
> along with this program.  If not, see <http://www.gnu.org/licenses/>.

