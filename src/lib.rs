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

    let mut a_trios = trios_of(a);
    let mut b_trios = trios_of(b);

    // Drop first part of a, last two parts of b
    // Just slice when initiating next stage?
    a_trios.remove(0);
    b_trios.pop();
    b_trios.pop();

    // Find indexes of matching trios
    // Could optimise by looking at number of shared letters and skipping more entries in the trio if no letters are shared
    // Consider itertools cartesian_product?
    // https://docs.rs/itertools/0.8.0/itertools/trait.Itertools.html#method.cartesian_product
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

pub fn portmanteau(a: &str, b: &str) -> Option<String> {
    if a.len() < MIN_WORD_SIZE || b.len() < MIN_WORD_SIZE {
        return None;
    }

    let by_trios = portmanteau_by_trios(a, b);
    if by_trios.is_some() {
        return by_trios;
    }

    if !has_vowel(a) || !has_vowel(b) {
        return None;
    }

    // Find locations of common vowels, but not those that are too close to the start or end
    let mut vowel_index_pair = None;
    for c in ['a', 'e', 'i', 'o', 'u'] {
        if let Some(a_index) = a[..a.len() - VOWEL_SEARCH_MARGIN].rfind(c) {
            if let Some(b_index) = b[VOWEL_SEARCH_MARGIN..].find(c) {
                vowel_index_pair = Some((a_index, b_index + VOWEL_SEARCH_MARGIN));
                //println!("Found matching vowel pair");
                break;
            }
        }
    }

    // If we didn't get any common vowels, we'll just go for any vowels really
    if vowel_index_pair.is_none() {
        //println!("Using any random vowels");
        vowel_index_pair = Some((
            // Get rightmost vowel of a
            a.rfind(&VOWELS[..]).unwrap(),
            // with leftmost vowel of b
            b.find(&VOWELS[..]).unwrap(),
        ));
    }

    let (a_end, b_start) = vowel_index_pair.unwrap();

    Some(format!("{}{}", &a[..a_end], &b[b_start..]))
}

#[cfg(test)]
mod internal_tests {
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
    #[should_panic]
    fn by_trios_panic_too_short() {
        portmanteau_by_trios("smol", "word");
    }

    #[test]
    fn by_matching_vowels() {
        assert_eq!(
            portmanteau("liquid", "slinky"),
            Some("liquinky".to_string())
        );
        assert_eq!(
            portmanteau("innovative", "madlad"),
            Some("innovadlad".to_string())
        );
        assert_eq!(portmanteau("space", "magic"), Some("spagic".to_string()));
        assert_eq!(portmanteau("crotch", "goblin"), Some("croblin".to_string()));
        assert_eq!(
            portmanteau("illegal", "beagle"),
            Some("illegagle".to_string())
        );
    }

    #[test]
    fn by_any_vowels() {
        assert_eq!(
            portmanteau("testicle", "crust"),
            Some("testiclust".to_string()),
        );
        assert_eq!(
            portmanteau("magical", "cheeses"),
            Some("magiceeses".to_string())
        );
        assert_eq!(
            portmanteau("crutch", "itches"),
            Some("crutches".to_string())
        );
        assert_eq!(
            portmanteau("squirrel", "acorn"),
            Some("squirracorn".to_string())
        );
        assert_eq!(
            portmanteau("pervert", "window"),
            Some("pervindow".to_string())
        );
    }
}
