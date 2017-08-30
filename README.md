# xkpwgen [![Current release][crates-badge]][crates] ![Maintenance as is][maintenance-badge] [![Build status][travis-badge]][travis]

[crates-badge]: https://img.shields.io/crates/v/xkpwgen.svg
[crates]: https://crates.io/crates/xkpwgen
[travis-badge]: https://img.shields.io/travis/lunaryorn/xkpwgen.rs/master.svg
[travis]: https://travis-ci.org/lunaryorn/xkpwgen.rs
[maintenance-badge]: https://img.shields.io/badge/maintenace-as--is-yellow.svg

Generate [XKCD 936](https://xkcd.com/936/) passwords:

![](http://imgs.xkcd.com/comics/password_strength.png)

## Install

```console
$ cargo install xkpwgen
```

## Use

Invoke `xkpwgen` to generate five passwords:

```console
$ xkpwgen
gains spate rush dine
stool added split skirt
venom foul slack rubs
leer merit ting slate
cribs flock stars help
```

Use `-l` to change the length of passwords, and `-n` to change the number of
passwords:

```console
$ xkpwgen -l 10 -n 2
goals tray guy mill mint cores focus kudos mares beady
ins hark sodas omit glove goofs spurt mash gait beer
```

See `xkpwgen --help` for more information.

## Words

xkpwgen uses the [pokerware wordlists][1] by Christopher Wellons.  His [blog
post][2] explains in detail how he collected these wordlists.  In essence the
default “formal” wordlist comes from formal publications whereas the alternative
“slang” wordlist collects popular words from Reddit comments.

[1]: https://github.com/skeeto/pokerware
[2]: http://nullprogram.com/blog/2017/07/27/

## License

Wordlist copyright (C) 2017 Christopher Wellons, released to [public
domain][pd].

[pd]: https://github.com/skeeto/pokerware/tree/89a8fec541fdbe04fe15b5ad0d7986019240f741

----

xkpwgen copyright (C) 2017  Sebastian Wiesner

xkpwgen is licensed under [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).
