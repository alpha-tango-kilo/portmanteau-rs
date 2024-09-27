#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! A portmanteau is a made up word derived as a combination of two other words,
//! e.g. "liquid" + "slinky" = "liquinky"
//!
//! It isn't always possible to produce a portmanteau from the input words
//! (there are some quality checks in place), so the exposed `portmanteau`
//! function returns an `Option<String>`. This is currently the only function
//! the crate exposes
//!
//! This library's initial implementation was largely inspired by the work of [jamcowl's portmanteau bot](https://github.com/jamcowl/PORTMANTEAU-BOT).
//! The full implementation is not available in their repository and over time
//! my implementation may differ from theirs in terms of approach and output (it
//! already does in some cases)
//!
//! Please do not hold myself (or any contributer to this repository)
//! accountable for any derogatory words or slurs you are able to have this
//! algorithm produce. There are no checks for bad language in place, and there
//! are no plans to add any. It is not my (or any contributer's) job to
//! determine what is or isn't offensive

use std::ops::Deref;

const MIN_WORD_SIZE: usize = 5;
const MATCHING_VOWEL_SEARCH_MARGIN: usize = 1;
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

/// Stores the vowel locations within a word (search direction set by
/// `from_left`)
///
///                   A     E        I        O        U
/// "helloski" -> `[None, Some(1), Some(7), Some(4), None]`
#[derive(Debug, Copy, Clone)]
struct VowelMap([Option<usize>; 5]);

impl VowelMap {
    fn from_ltr(word: &str) -> Self {
        let substring = &word[..word.len() - MATCHING_VOWEL_SEARCH_MARGIN];
        VowelMap(VOWELS.map(|vowel| substring.find(vowel)))
    }

    fn from_rtl(word: &str) -> Self {
        let substring = &word[MATCHING_VOWEL_SEARCH_MARGIN..];
        VowelMap(VOWELS.map(|vowel| {
            substring.rfind(vowel)
            // Make index relative to the whole word
            .map(|index| index + MATCHING_VOWEL_SEARCH_MARGIN)
        }))
    }
}

impl Deref for VowelMap {
    type Target = [Option<usize>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn trios_of(
    string: &str,
) -> impl DoubleEndedIterator<Item = &str> + ExactSizeIterator {
    // Shouldn't happen in real world so leaving as debug assertion
    // The for loop will panic if it happens
    debug_assert!(
        string.len() >= 3,
        "Trios shouldn't be asked for from words with less than 3 letters"
    );

    (0..string.len() - 2).map(move |index| &string[index..index + 3])
}

fn portmanteau_by_trios(a: &str, b: &str) -> Option<String> {
    debug_assert!(
        a.len() >= MIN_WORD_SIZE && b.len() >= MIN_WORD_SIZE,
        "Less than {} letter words should have already been eliminated",
        MIN_WORD_SIZE
    );

    let a_trios = trios_of(a).skip(1);

    // Find indexes of matching trios
    // Could optimise by looking at number of shared letters and skipping more
    // entries in the trio if no letters are shared
    a_trios
        .enumerate()
        // .rev here and in b_trios prioritises finding longer portmaneau by
        // searching from the end of `a` and the start of `b`
        .rev()
        // Cartesian product with b_trios
        .flat_map(|a_trio_tup| {
            let b_trios = trios_of(b).enumerate().rev().skip(2).rev();
            b_trios.map(move |b_trio_tup| (a_trio_tup, b_trio_tup))
        })
        .find(|((_, a_trio), (_, b_trio))| a_trio == b_trio)
        .map(|((a_pos, _), (b_pos, _))| {
            format!("{}{}", &a[..a_pos + 1], &b[b_pos..])
        })
}

#[inline]
fn validate(s: &str) -> bool {
    s.len() >= MIN_WORD_SIZE && s.chars().all(|c| c.is_ascii_lowercase())
}

/// This function creates a portmanteau of the two given words if possible
///
/// Both inputs given should be lowercase single words, without punctuation, and
/// 5 or more letters in length. Doing so would result in receiving `None`
///
/// # Examples
///
/// ```
/// use portmanteau::portmanteau;
///
/// let something = portmanteau("fluffy", "turtle");
/// assert_eq!(something, Some(String::from("flurtle")));
///
/// let nothing = portmanteau("tiny", "word");
/// assert_eq!(nothing, None);
/// ```
pub fn portmanteau(left_word: &str, right_word: &str) -> Option<String> {
    // Step 1: validate input strings to be acceptable
    if !(validate(left_word) && validate(right_word)) {
        return None;
    }

    // Step 2: Try and get a portmanteau by trios
    portmanteau_by_trios(left_word, right_word)
        .or_else(|| {
            // Step 3: Try and join on vowels (ideally a matching pair)
            let left_vowels = VowelMap::from_rtl(left_word);
            let right_vowels = VowelMap::from_ltr(right_word);

            let mut chosen_left_vowel_index: Option<usize> = None;
            let mut chosen_right_vowel_index: Option<usize> = None;
            for (left_vowel_index, right_vowel_index) in
                left_vowels.iter().zip(right_vowels.deref())
            {
                match (left_vowel_index, right_vowel_index) {
                    (Some(_), Some(_)) => {
                        // Matching vowels is best-case, immediately break & use
                        // this
                        chosen_left_vowel_index = *left_vowel_index;
                        chosen_right_vowel_index = *right_vowel_index;
                        break;
                    },
                    (Some(left_index), None) => chosen_left_vowel_index
                        .replace_if(|inner| left_index > inner, *left_index),
                    (None, Some(right_index)) => chosen_right_vowel_index
                        .replace_if(|inner| right_index < inner, *right_index),
                    (None, None) => {},
                }
            }
            chosen_left_vowel_index.zip(chosen_right_vowel_index).map(
                |(left_vowel_index, right_vowel_index)| {
                    // println!(
                    //     "{left_vowels:?} <- {left_word:?} -> \
                    //      {left_vowel_index}"
                    // );
                    // println!(
                    //     "{right_vowels:?} <- {right_word:?} -> \
                    //      {right_vowel_index}"
                    // );
                    format!(
                        "{}{}",
                        &left_word[..left_vowel_index],
                        &right_word[right_vowel_index..],
                    )
                },
            )
        })
        .filter(|portmanteau| {
            !(left_word.contains(portmanteau)
                || right_word.contains(portmanteau))
        })
}

trait OptionExt<T> {
    fn replace_if<P: FnOnce(&T) -> bool>(&mut self, predicate: P, value: T);
}

impl<T> OptionExt<T> for Option<T> {
    fn replace_if<P: FnOnce(&T) -> bool>(&mut self, predicate: P, value: T) {
        match self {
            Some(old) if predicate(old) => *self = Some(value),
            Some(_) => {},
            None => *self = Some(value),
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::*;

    #[test]
    fn trios() {
        let solutions: [&str; 3] = ["abc", "bcd", "cde"];

        assert_eq!(trios_of("abc").collect::<Vec<_>>(), solutions[0..1]);
        assert_eq!(*trios_of("abcd").collect::<Vec<_>>(), solutions[0..2]);
        assert_eq!(*trios_of("abcde").collect::<Vec<_>>(), solutions[0..3]);
    }

    #[test]
    #[should_panic]
    fn trios_panic_too_short() {
        trios_of("").for_each(drop);
    }

    #[test]
    fn by_trios() {
        assert_eq!(
            portmanteau_by_trios("chrome", "promise"),
            Some("chromise".to_string())
        );
        assert_eq!(
            portmanteau_by_trios("crime", "experimental"),
            Some("crimental".to_string())
        );
        assert_eq!(
            portmanteau_by_trios("pleasurable", "breaststroke"),
            Some("pleaststroke".to_string())
        );
        assert_eq!(
            portmanteau_by_trios("unthreatening", "recreation"),
            Some("unthreation".to_string())
        );
    }

    #[test]
    fn by_trios_no_vowels() {
        assert_eq!(
            portmanteau_by_trios("sdfghjk", "qwrdfgvbnm"),
            Some("sdfgvbnm".to_string())
        );
        assert_eq!(
            portmanteau("sdfghjk", "qwrdfgvbnm"),
            Some("sdfgvbnm".to_string()),
            "The portmanteau function is rejecting due to lack of vowels too \
             early!"
        );
    }

    #[test]
    #[should_panic]
    fn by_trios_panic_too_short() {
        portmanteau_by_trios("smol", "word");
    }

    #[test]
    fn validation() {
        assert!(validate("hello"));
        assert!(!validate("Hello"));
        assert!(!validate("smol"));
        assert!(!validate("symbols!"));
        assert!(!validate("s p a c e s"));
        assert!(!validate("😃😂😉🤩🙄"));
        assert!(!validate("accénts"))
    }
}
