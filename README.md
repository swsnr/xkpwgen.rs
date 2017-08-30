# xkpwgen [![Current release][crates-badge]][crates] [![Build status][travis-badge]][travis]

[crates-badge]: https://img.shields.io/crates/v/xkpwgen.svg
[crates]: https://crates.io/crates/xkpwgen
[travis-badge]: https://img.shields.io/travis/lunaryorn/xkpwgen.rs/master.svg
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

xkpwgen includes the [EFF long wordlist][1] which contains 7776 common English
words between three and nine characters in length, at an average of seven
characters per word.

The wordlist is provided free of charge by the EFF and may be freely distributed
under the [CC BY 3.0 US license](https://www.eff.org/copyright).  If you like
xkpwgen please consider [supporting the EFF](https://supporters.eff.org)!

[1]: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases

## License

Wordlist copyright (C) 2016 EFF

The wordlist used by xkpwgen may be freely distributed at will under the
[Creative Commons Attribution License][cc-by] (CC BY 3.0 US).  See
<https://www.eff.org/copyright> for more information about the EFF's copyright
policy.

[cc-by]: http://creativecommons.org/licenses/by/3.0/us/

----

xkpwgen copyright (C) 2017  Sebastian Wiesner

xkpwgen is licensed under [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).
