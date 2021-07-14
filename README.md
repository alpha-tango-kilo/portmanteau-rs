# Portmanteau

[![Crates.io](https://img.shields.io/crates/v/portmanteau.svg)](https://crates.io/crates/portmanteau)
[![Documentation](https://docs.rs/portmanteau/badge.svg)](https://docs.rs/portmanteau)
![](https://img.shields.io/badge/unsafe-forbidden-darkgreen.svg)

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

## Explanation

At a high level, this is how the algorithm works:

1. Check the inputs are good
2. See if there are any shared 3 letter combinations (trios)*. Join here if so
3. See if there are any shared vowels*. Join here if so
4. Join the rightmost vowel of the first word to the leftmost vowel of the second word

*: The start of the first word and the end of the last word is cut off for these operations to avoid low quality output

## Installation

To install and use `portmanteau` as a binary application on your system, you can simply run:

```shell
cargo install portmanteau
```

You will need the `--force` flag if you are updating

For usage instructions, please refer to `portmanteau --help`

## Roadmap

* Optimise/Enhance
  * See other branches for attempted optimisations where they have benchmarked more poorly (hall of shame, if you will)
* ~~Better quality control~~ (complete in v0.2.0)
* Benchmark (added in v0.2.0)
  * Benchmarks per code path (trio matching, vowel matching, random vowels)
* More thorough testing
* Builder pattern for more configurable generation
* ~~CLI tool~~ (available in workspace `portmanteau-bin`)

## Licensing

Much the same as Rust is, this library is dual-licensed under MIT or Apache 2.0 at your choosing
