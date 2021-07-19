use assert_cmd::Command;

const EXECUTABLE: &str = "portmanteau";

fn get_bin() -> Command {
    Command::cargo_bin(EXECUTABLE).expect("Executable's name has changed?")
}

#[test]
fn version() {
    get_bin()
        .arg("-v")
        .assert()
        .stdout(format!("v{}\n", env!("CARGO_PKG_VERSION")))
        .success();
    get_bin()
        .arg("--version")
        .assert()
        .stdout(format!("v{}\n", env!("CARGO_PKG_VERSION")))
        .success();
}

#[test]
fn help() {
    get_bin().arg("-h").assert().success();
    get_bin().arg("--help").assert().success();
}

#[test]
fn word_splits() {
    get_bin()
        .args(&["-w", ".", "liquid.slinky"])
        .assert()
        .stdout("liquinky\n")
        .stderr("")
        .success();
    get_bin()
        .args(&["-w", ".-.", "liquid.-.slinky"])
        .assert()
        .stdout("liquinky\n")
        .stderr("")
        .success();
    get_bin()
        .args(&["-w", ",", "-"])
        .write_stdin("liquid,slinky")
        .assert()
        .stdout("liquinky\n")
        .stderr("")
        .success();
}

#[test]
fn line_splits() {
    get_bin()
        .args(&["-l", ".", "-"])
        .write_stdin("liquid slinky.innovative madlad")
        .assert()
        .stdout("liquinky\ninnovadlad\n")
        .stderr("");
    get_bin()
        .args(&["-l", "\t", "-"])
        .write_stdin("liquid slinky\tinnovative madlad")
        .assert()
        .stdout("liquinky\ninnovadlad\n")
        .stderr("");
}

#[test]
fn bad_line_split() {
    get_bin()
        .args(&["-l", ",\n", "-"])
        .write_stdin("liquid slinky,\ninnovative madlad")
        .assert()
        .stdout("")
        .stderr("Line delimiter can only be a single character\n")
        .code(2);
}

#[test]
fn args_mode() {
    get_bin()
        .args(&["liquid", "slinky"])
        .assert()
        .stdout("liquinky\n")
        .stderr("")
        .success();
    get_bin().arg("liquid").assert().code(2);
    get_bin()
        .args(&["liquid", "slinky", "dogs"])
        .assert()
        .stdout("liquinky\n")
        .stderr("More words than expected on line\n")
        .success();
}

#[test]
fn stdin_mode() {
    get_bin()
        .arg("-")
        .write_stdin("liquid slinky")
        .assert()
        .stdout("liquinky\n")
        .stderr("")
        .success();
}

mod args_mode_errors {
    use crate::*;

    #[test]
    fn insufficient_args() {
        get_bin()
            .arg("liquid")
            .assert()
            .stderr("Insufficient arguments provided, expected 2\n")
            .code(2);
        get_bin()
            .assert()
            .stderr("Insufficient arguments provided, expected 2\n")
            .code(2);
        get_bin()
            .args(&["-w", "."])
            .assert()
            .stderr("Insufficient arguments provided, expected 1\n")
            .code(2);
    }

    #[test]
    fn bad_word_split() {
        get_bin()
            .args(&["-w", ",", "liquidslinky"])
            .assert()
            .stderr("Split \",\" failed to produce at least two parts\n")
            .code(2);
    }

    #[test]
    fn none_produced() {
        get_bin()
            .args(&["wet", "dog"])
            .assert()
            .stderr("\"wet\" and \"dog\" did not produce a portmanteau\n")
            .code(1);
        get_bin()
            .args(&["-w", ".", "wet.dog"])
            .assert()
            .stderr("\"wet\" and \"dog\" did not produce a portmanteau\n")
            .code(1);
    }
}
