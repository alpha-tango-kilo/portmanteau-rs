# Portmanteau

A portmanteau is a made up word derived as a combination of two other words, e.g. "liquid" + "slinky" = "liquinky"

It isn't always possible to produce a portmanteau from the input words (there are some quality checks in place), so the exposed `portmanteau` function returns an `Option<String>`

The general guidelines to get a result are as follows:
* Both words need to be over 5 letters long
* Both words must be lowercase
* Both words must have vowels

This library's initial implementation was largely inspired by the work of [jamcowl's portmanteau bot](https://github.com/jamcowl/PORTMANTEAU-BOT).
The full implementation is not available in their repository and over time my implementation may differ from theirs in terms of approach and output (it already does in some cases)

Please do not hold myself (or any contributer to this repository) accountable for any derogatory words or slurs you are able to have this algorithm produce.
There are no checks for bad language in place, and there are no plans to add any.
It is not my (or any contributer's) job to determine what is or isn't offensive

## Roadmap

* Optimise/Enhance
* Better quality control
* More thorough testing
* Builder pattern for more configurable generation
* CLI tool???

## Licensing

Much the same as Rust is, this library is dual-licensed under MIT or Apache 2.0 at your choosing
