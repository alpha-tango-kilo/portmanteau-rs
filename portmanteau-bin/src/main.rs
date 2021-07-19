#![forbid(unsafe_code)]

use portmanteau::portmanteau;
use portmanteau_bin::BinError::*;
use portmanteau_bin::*;
use std::io::BufRead;
use std::{io, process};

const HELP: &str = "\
portmanteau

USAGE:
  portmanteau [OPTIONS] [WORD 1] [WORD 2]           Words to combine given as arguments
  portmanteau [OPTIONS] -                           Words to combine taken from STDIN line-by-line

OPTIONS:
  -w [delimiter], --word-split [delimiter]          Specify the character(s) between the two words being input
  -h, --help                                        Access this help text
  -v, --version                                     Print the program version

EXIT CODES:
  0                                                 All good
  1                                                 No portmanteau produced (in arguments mode)
  2                                                 User error
  3                                                 Program error
";

#[inline]
fn print_help() {
    println!("{}", HELP);
    process::exit(0);
}

type Result<T> = std::result::Result<T, BinError>;

fn main() {
    if let Err(what) = app() {
        eprintln!("{}", what);
        process::exit(what.get_exit_code())
    }
}

fn app() -> Result<()> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print_help();
    } else if pargs.contains(["-v", "--version"]) {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let config = RuntimeConfig::from_pico_args(&mut pargs);

    if pargs.contains("-") {
        // STDIN mode
        //eprintln!("STDIN mode");
        io::stdin().lock().lines().for_each(|line| {
            // STDIN mode handles errors line-by-line and just prints them without aborting
            if let Err(warning) = stdin_line(&config, line) {
                eprintln!("{}", warning);
            }
        });
    } else {
        // Args mode
        //eprintln!("Args mode");
        args_mode(&config, pargs)?;
    }
    Ok(())
}

fn stdin_line(config: &RuntimeConfig, line: io::Result<String>) -> Result<()> {
    let line = line?;
    let mut words = line.split(&config.word_split);
    let a = words.next().ok_or(InsufficientArguments(None))?;
    let b = words.next().ok_or(InsufficientArguments(None))?;

    if words.next().is_some() {
        eprintln!("More words than expected on line");
    }

    if let Some(pm) = portmanteau(a, b) {
        println!("{}", pm);
    }
    Ok(())
}

fn args_mode(config: &RuntimeConfig, pargs: pico_args::Arguments) -> Result<()> {
    let remaining_args = pargs.finish();

    if config.is_split_whitespace() {
        // Expect two args
        if remaining_args.len() > 2 {
            eprintln!("More words than expected on line");
        }
        let a = &remaining_args
            .get(0)
            .ok_or(InsufficientArguments(Some(2)))?
            .to_string_lossy();
        let b = &remaining_args
            .get(1)
            .ok_or(InsufficientArguments(Some(2)))?
            .to_string_lossy();
        match portmanteau(a, b) {
            Some(pm) => println!("{}", pm),
            None => return Err(NoneProduced((a.to_string(), b.to_string()))),
        }
    } else {
        // Expect one arg
        if remaining_args.len() > 1 {
            eprintln!("More words than expected on line");
        }
        let s = remaining_args
            .get(0)
            .ok_or(InsufficientArguments(Some(1)))?
            .to_string_lossy();
        let mut s_iter = s.split(&config.word_split);
        let a = s_iter.next().ok_or(BadSplit(config.word_split.clone()))?;
        let b = s_iter.next().ok_or(BadSplit(config.word_split.clone()))?;
        match portmanteau(a, b) {
            Some(pm) => println!("{}", pm),
            None => return Err(NoneProduced((a.into(), b.into()))),
        }
    }
    Ok(())
}
