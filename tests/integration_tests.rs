use portmanteau::*;

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

#[test]
fn output_matches_input() {
    assert_eq!(portmanteau("swords", "words"), None);
}

#[test]
fn check_csv() {
    csv::Reader::from_path("benches/input_pairs.csv")
        .expect("unable to find input file for benchmark")
        .records()
        .map(|record| {
            let record = record.expect("failed to parse input file");
            let left_word = String::from(record.get(0).unwrap());
            let right_word = String::from(record.get(1).unwrap());
            (left_word, right_word)
        })
        .for_each(|(left_word, right_word)| {
            assert!(
                portmanteau(&left_word, &right_word).is_some(),
                "{:?} + {:?} did not make a portmanteau",
                left_word,
                right_word,
            );
        });
}
