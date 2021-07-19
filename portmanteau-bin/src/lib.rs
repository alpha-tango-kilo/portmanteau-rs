use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub struct RuntimeConfig {
    pub word_split: String,
}

impl RuntimeConfig {
    pub fn from_pico_args(pargs: &mut pico_args::Arguments) -> Self {
        let word_split = pargs
            .value_from_str(["-w", "--word-split"])
            .unwrap_or(RuntimeConfig::default().word_split);
        RuntimeConfig {
            word_split,
        }
    }

    #[inline]
    pub fn is_split_whitespace(&self) -> bool {
        self.word_split.trim().is_empty()
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig {
            word_split: String::from(' '),
        }
    }
}

#[derive(Debug)]
pub enum BinError {
    InsufficientArguments(Option<usize>), // Expected number isn't always known/applicable, hence the Option
    //ArgumentParsing(pico_args::Error),
    BadSplit(String), // TODO: use reference?
    StdinEnd(io::Error),
    NoneProduced((String, String)), // TODO: use reference?
}

impl BinError {
    pub fn get_exit_code(&self) -> i32 {
        use BinError::*;
        match self {
            InsufficientArguments(_) => 2,
            BadSplit(_) => 2,
            StdinEnd(_) => 3,
            NoneProduced(_) => 1,
        }
    }
}

impl fmt::Display for BinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinError::*;
        match self {
            InsufficientArguments(n) => match n {
                Some(n) => write!(f, "Insufficient arguments provided, expected {}", *n),
                None => write!(f, "Couldn't find two words to combine"),
            },
            //ArgumentParsing(pico_err) => write!(f, "Problem parsing arguments ({})", pico_err),
            BadSplit(split) => write!(f, "Split {:?} failed to produce at least two parts", split),
            StdinEnd(io_err) => write!(f, "STDIN read ended with error ({})", io_err),
            NoneProduced((a, b)) => write!(f, "{:?} and {:?} did not produce a portmanteau", a, b),
        }
    }
}

impl Error for BinError {}

/*
impl From<pico_args::Error> for PortmanteauBinError {
    fn from(pico_err: pico_args::Error) -> Self {
        PortmanteauBinError::ArgumentParsing(pico_err)
    }
}
*/

impl From<io::Error> for BinError {
    fn from(io_error: io::Error) -> Self {
        BinError::StdinEnd(io_error)
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::RuntimeConfig;
    use pico_args::Arguments;
    use std::ffi::OsString;

    // https://github.com/RazrFalcon/pico-args/blob/3014e061ee8fe54ecbab8a5fa6e78ccb5c4b8b79/tests/tests.rs#L6-L8
    fn to_pico_vec(args: &[&str]) -> Vec<OsString> {
        args.iter().map(|s| s.to_string().into()).collect()
    }

    #[test]
    fn default() {
        let mut pargs = Arguments::from_vec(to_pico_vec(&[]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(config.word_split, RuntimeConfig::default().word_split);
    }

    #[test]
    fn short_split() {
        let mut pargs = Arguments::from_vec(to_pico_vec(&["-w", "."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(&config.word_split, ".");
    }

    #[test]
    fn long_split() {
        let mut pargs = Arguments::from_vec(to_pico_vec(&["--word-split", "."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(&config.word_split, ".");
    }

    #[test]
    fn string_split() {
        let mut pargs = Arguments::from_vec(to_pico_vec(&["--word-split", ".-."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(&config.word_split, ".-.");
    }

    #[test]
    fn multiple_splits() {
        // Short option is checked first
        let mut pargs = Arguments::from_vec(to_pico_vec(&["--word-split", ".", "-w", ","]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(&config.word_split, ",");

        let mut pargs = Arguments::from_vec(to_pico_vec(&["-w", ".", "--word-split", ","]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(&config.word_split, ".");

        // First choice is taken when multiple identical flags are given
        let mut pargs = Arguments::from_vec(to_pico_vec(&["-w", ".", "-w", ","]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(&config.word_split, ".");

        let mut pargs = Arguments::from_vec(to_pico_vec(&["--word-split", ".", "--word-split", ","]));
        let config = RuntimeConfig::from_pico_args(&mut pargs);
        assert_eq!(&config.word_split, ".");
    }
}
