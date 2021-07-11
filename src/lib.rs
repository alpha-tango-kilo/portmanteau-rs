#![deny(missing_docs)]

//! A portmanteau is a made up word derived as a combination of two other words, e.g. "liquid" + "slinky" = "liquinky"
//!
//! It isn't always possible to produce a portmanteau from the input words (there are some quality checks in place), so the exposed `portmanteau` function returns an `Option<String>`.
//! This is currently the only function the crate exposes
//!
//! This library's initial implementation was largely inspired by the work of [jamcowl's portmanteau bot](https://github.com/jamcowl/PORTMANTEAU-BOT).
//! The full implementation is not available in their repository and over time my implementation may differ from theirs in terms of approach and output (it already does in some cases)
//!
//! Please do not hold myself (or any contributer to this repository) accountable for any derogatory words or slurs you are able to have this algorithm produce.
//! There are no checks for bad language in place, and there are no plans to add any.
//! It is not my (or any contributer's) job to determine what is or isn't offensive

use smallvec::SmallVec;

const MIN_WORD_SIZE: usize = 5;
const EXPECTED_TRIOS: usize = 3;
const VOWEL_SEARCH_MARGIN: usize = 1;
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

// Expects to always take lowercase `s`
fn has_vowel(s: &str) -> bool {
    debug_assert!(
        !s.chars().any(|c| c.is_uppercase()),
        "Uppercase characters should have been accounted for before has_vowel"
    );
    s.contains(&VOWELS[..])
}

fn trios_of(s: &str) -> SmallVec<[&str; EXPECTED_TRIOS]> {
    // Shouldn't happen in real world so leaving as debug assertion
    // The for loop will panic if it happens
    debug_assert!(
        s.len() >= 3,
        "Trios shouldn't be asked for from words with less than 3 letters"
    );

    let mut trios = SmallVec::new();
    for n in 0..s.len() - 2 {
        trios.push(&s[n..n + 3]);
    }
    trios
}

fn portmanteau_by_trios(a: &str, b: &str) -> Option<String> {
    debug_assert!(
        a.len() >= MIN_WORD_SIZE && b.len() >= MIN_WORD_SIZE,
        "Less than {} letter words should have already been eliminated",
        MIN_WORD_SIZE
    );

    let a_trios = &trios_of(a)[1..];
    let b_trios = &trios_of(b);
    let b_trios = &b_trios[..b_trios.len() - 2];

    // Find indexes of matching trios
    // Could optimise by looking at number of shared letters and skipping more entries in the trio if no letters are shared
    // Could try aho-corasick
    for (a_pos, a_trio) in a_trios.iter().enumerate() {
        for (b_pos, b_trio) in b_trios.iter().enumerate() {
            if a_trio == b_trio {
                //println!("Found matching trios");
                return Some(format!("{}{}", &a[..a_pos + 1], &b[b_pos..]));
            }
        }
    }
    None
}

#[inline]
fn validate(s: &str) -> bool {
    s.len() >= MIN_WORD_SIZE && s.chars().all(|c| c.is_ascii_lowercase())
}

fn portmanteau_by_common_vowels(a: &str, b: &str) -> Option<String> {
    // Find locations of common vowels, but not those that are too close to the start or end
    for c in VOWELS {
        if let Some(a_index) = a[..a.len() - VOWEL_SEARCH_MARGIN].rfind(c) {
            if let Some(b_index) = b[VOWEL_SEARCH_MARGIN..].find(c) {
                //println!("Found matching vowel pair");
                return Some(format!(
                    "{}{}",
                    &a[..a_index],
                    &b[b_index + VOWEL_SEARCH_MARGIN..]
                ));
            }
        }
    }
    None
}

fn portmanteau_by_any_vowels(a: &str, b: &str) -> Option<String> {
    // Get rightmost vowel of a
    let a_end = a.rfind(&VOWELS[..]).unwrap();
    // with leftmost vowel of b
    let b_start = b.find(&VOWELS[..]).unwrap();
    Some(format!("{}{}", &a[..a_end], &b[b_start..]))
}

/// This function creates a portmanteau of the two given words if possible
///
/// Both inputs given should be lowercase single words, without punctuation, and 5 or more letters in length.
/// Doing so would result in receiving `None`
///
/// # Examples
///
/// ```
/// use portmanteau::portmanteau;
///
///     let something = portmanteau("fluffy", "turtle");
///     assert_eq!(
///     something,
///     Some(String::from("flurtle"))
///     );
///
///     let nothing = portmanteau("tiny", "word");
///     assert_eq!(
///     nothing,
///     None
///     );
/// ```

pub fn portmanteau(a: &str, b: &str) -> Option<String> {
    // Step 1: validate input strings to be acceptable
    if !(validate(a) && validate(b)) {
        return None;
    }

    // Step 2: Try and get a portmanteau by trios
    let output = portmanteau_by_trios(a, b);

    // Step 3: Check for presence of vowels
    if output.is_none() && !(has_vowel(a) && has_vowel(b)) {
        return None;
    }

    // Step 4: Match common vowels
    output
        .or_else(|| portmanteau_by_common_vowels(a, b))
        // Step 5: Match any two vowels
        .or_else(|| portmanteau_by_any_vowels(a, b))
        // Step 6: Make sure we aren't outputting either input word
        .and_then(|pm| {
            if !pm.eq(a) && !pm.eq(b) {
                Some(pm)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod unit_tests {
    use crate::*;

    #[test]
    fn vowels() {
        assert!(has_vowel("hello"));
        assert!(has_vowel("hi"));
        assert!(has_vowel("hallo"));
        assert!(has_vowel("howdy"));
        assert!(!has_vowel("why"));
        assert!(!has_vowel("rhythm"));
    }

    #[test]
    #[should_panic]
    fn vowels_panic_uppercase() {
        has_vowel("HI");
    }

    #[test]
    fn trios() {
        let solutions: [&str; 3] = ["abc", "bcd", "cde"];

        assert_eq!(*trios_of("abc"), solutions[0..1]);
        assert_eq!(*trios_of("abcd"), solutions[0..2]);
        assert_eq!(*trios_of("abcde"), solutions[0..3]);
    }

    #[test]
    #[should_panic]
    fn trios_panic_too_short() {
        trios_of("");
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
            "The portmanteau function is rejecting due to lack of vowels too early!"
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
