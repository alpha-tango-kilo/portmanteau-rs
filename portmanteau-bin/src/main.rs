#![forbid(unsafe_code)]

use portmanteau::portmanteau;
use std::io::BufRead;
use std::{io, process};

const HELP: &str = "\
portmanteau

USAGE:
  portmanteau [OPTIONS] [WORD 1] [WORD 2]           Words to combine given as arguments
  portmanteau [OPTIONS] -                           Words to combine taken from STDIN line-by-line

OPTIONS:
  -h, --help                                        Access this help text
  -v, --version                                     Print the program version

EXIT CODES:
  0                                                 All good
  1                                                 No portmanteau produced (in arguments mode)
  2                                                 Unexpected error
";

#[inline]
fn print_help() {
    println!("{}", HELP);
    process::exit(0);
}

fn main() -> Result<(), pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print_help();
    } else if pargs.contains(["-v", "--version"]) {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if pargs.contains("-") {
        // STDIN mode
        //eprintln!("STDIN mode");
        let status = io::stdin()
            .lock()
            .lines()
            .map(|s| {
                let s = s?;
                let mut words = s.split(' ');
                let a = match words.next() {
                    Some(s) => s,
                    None => {
                        eprintln!("Words not found in line");
                        return Ok(());
                    },
                };
                let b = match words.next() {
                    Some(s) => s,
                    None => {
                        eprintln!("Second word not found in line");
                        return Ok(());
                    },
                };

                if words.next().is_some() {
                    eprintln!("More words than expected on line");
                }

                if let Some(pm) = portmanteau(a, b) {
                    println!("{}", pm);
                }
                Ok(())
            })
            .collect::<io::Result<()>>();
        if let Err(why) = status {
            eprintln!("STDIN read ended with error ({})", why);
            process::exit(2);
        }
    } else {
        // Args mode
        //eprintln!("Args mode");
        let a_option = pargs.subcommand()?;
        let b_option = pargs.subcommand()?;
        let extras = pargs.finish();

        if !extras.is_empty() {
            eprintln!("WARNING: extra arguments provided ({:?})", extras);
        }

        match (a_option, b_option) {
            (Some(a), Some(b)) => match portmanteau(&a, &b) {
                Some(pm) => println!("{}", pm),
                None => process::exit(1),
            },
            (None, _) => {
                // No arguments given, default to help
                print_help();
            },
            _ => {
                eprintln!(
                    "Failed to process arguments - expected to be given two words to combine\n\
            Use `portmanteau --help` for help"
                );
                process::exit(2);
            }
        }
    }

    Ok(())
}
