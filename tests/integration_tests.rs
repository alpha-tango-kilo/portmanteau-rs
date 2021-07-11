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
