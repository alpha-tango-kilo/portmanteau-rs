use std::{error::Error, fmt, io, str::Utf8Error};

use pico_args::Error::Utf8ArgumentParsingFailed;

type Result<T> = std::result::Result<T, BinError>;

#[derive(Debug)]
pub struct RuntimeConfig {
    pub word_split: String,
    pub line_split: char,
}

impl RuntimeConfig {
    pub fn from_pico_args(pargs: &mut pico_args::Arguments) -> Result<Self> {
        let word_split = pargs
            .value_from_str(["-w", "--word-split"])
            .unwrap_or(RuntimeConfig::default().word_split);
        let line_split = match pargs.value_from_str(["-l", "--line-split"]) {
            Ok(c) => c,
            Err(Utf8ArgumentParsingFailed { .. }) => {
                return Err(BinError::BadLineSplit)
            },
            Err(_) => RuntimeConfig::default().line_split,
        };

        Ok(RuntimeConfig {
            word_split,
            line_split,
        })
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
            line_split: '\n',
        }
    }
}

#[derive(Debug)]
pub enum BinError {
    InsufficientArguments(Option<usize>), /* Expected number isn't always
                                           * known/applicable, hence the
                                           * Option */
    //ArgumentParsing(pico_args::Error),
    BadWordSplit(String), // TODO: use reference?
    BadLineSplit,
    StdinEnd(io::Error),
    NoneProduced((String, String)), // TODO: use reference?
    DecodeStdin(Utf8Error),
}

impl BinError {
    pub fn get_exit_code(&self) -> i32 {
        use BinError::*;
        match self {
            InsufficientArguments(_) => 2,
            BadWordSplit(_) => 2,
            BadLineSplit => 2,
            StdinEnd(_) => 3,
            NoneProduced(_) => 1,
            DecodeStdin(_) => 3,
        }
    }
}

impl fmt::Display for BinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinError::*;
        match self {
            InsufficientArguments(n) => match n {
                Some(n) => write!(
                    f,
                    "Insufficient arguments provided, expected {}",
                    *n
                ),
                None => write!(f, "Couldn't find two words to combine"),
            },
            //ArgumentParsing(pico_err) => write!(f, "Problem parsing arguments
            // ({})", pico_err),
            BadWordSplit(split) => {
                write!(
                    f,
                    "Split {:?} failed to produce at least two parts",
                    split
                )
            },
            BadLineSplit => {
                write!(f, "Line delimiter can only be a single character")
            },
            StdinEnd(io_err) => {
                write!(f, "STDIN read ended with error ({})", io_err)
            },
            NoneProduced((a, b)) => {
                write!(f, "{:?} and {:?} did not produce a portmanteau", a, b)
            },
            DecodeStdin(utf_err) => {
                write!(f, "Failed to read STDIN with given split ({})", utf_err)
            },
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

impl From<std::str::Utf8Error> for BinError {
    fn from(utf_error: Utf8Error) -> Self {
        BinError::DecodeStdin(utf_error)
    }
}

#[cfg(test)]
mod unit_tests {
    use std::ffi::OsString;

    use pico_args::Arguments;

    use crate::RuntimeConfig;

    // https://github.com/RazrFalcon/pico-args/blob/3014e061ee8fe54ecbab8a5fa6e78ccb5c4b8b79/tests/tests.rs#L6-L8
    fn to_pico_vec(args: &[&str]) -> Vec<OsString> {
        args.iter().map(|s| s.to_string().into()).collect()
    }

    #[test]
    fn default() {
        let mut pargs = Arguments::from_vec(to_pico_vec(&[]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(config.word_split, RuntimeConfig::default().word_split);
        assert_eq!(config.line_split, RuntimeConfig::default().line_split)
    }

    #[test]
    fn short_word_split() {
        let mut pargs = Arguments::from_vec(to_pico_vec(&["-w", "."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(&config.word_split, ".");
    }

    #[test]
    fn long_word_split() {
        let mut pargs =
            Arguments::from_vec(to_pico_vec(&["--word-split", "."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(&config.word_split, ".");
    }

    #[test]
    fn string_word_split() {
        let mut pargs =
            Arguments::from_vec(to_pico_vec(&["--word-split", ".-."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(&config.word_split, ".-.");
    }

    #[test]
    fn short_line_split() {
        let mut pargs = Arguments::from_vec(to_pico_vec(&["-l", "."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(config.line_split, '.');
    }

    #[test]
    fn long_line_split() {
        let mut pargs =
            Arguments::from_vec(to_pico_vec(&["--line-split", "."]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(config.line_split, '.');
    }

    #[test]
    fn multiple_splits() {
        // Short option is checked first
        let mut pargs =
            Arguments::from_vec(to_pico_vec(&["--word-split", ".", "-w", ","]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(&config.word_split, ",");

        let mut pargs =
            Arguments::from_vec(to_pico_vec(&["-w", ".", "--word-split", ","]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(&config.word_split, ".");

        // First choice is taken when multiple identical flags are given
        let mut pargs =
            Arguments::from_vec(to_pico_vec(&["-w", ".", "-w", ","]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(&config.word_split, ".");

        let mut pargs = Arguments::from_vec(to_pico_vec(&[
            "--word-split",
            ".",
            "--word-split",
            ",",
        ]));
        let config = RuntimeConfig::from_pico_args(&mut pargs).unwrap();
        assert_eq!(&config.word_split, ".");
    }
}
