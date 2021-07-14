use std::process;
use portmanteau::portmanteau;

const HELP: &str = "\
portmanteau

USAGE:
  portmanteau [OPTIONS] [WORD 1] [WORD 2]           Words to combine given as arguments
  portmanteau [OPTIONS] -                           Words to combine taken from STDIN

OPTIONS:
  -h, --help                                        Access this help text
  -v, --version                                     Print the program version

EXIT CODES:
  0                                                 All good
  1                                                 No portmanteau produced (in arguments mode)
  2                                                 Unexpected error
";

fn main() -> Result<(), pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        println!("{}", HELP);
        process::exit(0);
    } else if pargs.contains(["-v", "--version"]) {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if pargs.contains("-") {
        // STDIN mode
        eprintln!("STDIN mode");
    } else {
        // Args mode
        let a_option = pargs.subcommand()?;
        let b_option = pargs.subcommand()?;
        let extras = pargs.finish();

        if !extras.is_empty() {
            eprintln!("WARNING: extra arguments provided ({:?})", extras);
        }

        match (a_option, b_option) {
            (Some(a), Some(b)) => {
                match portmanteau(&a, &b) {
                    Some(pm) => println!("{}", pm),
                    None => process::exit(1),
                }
            },
            _ => {
                eprintln!("Failed to process arguments - expected to be given two words to combine\
            Use `portmanteau --help` for help");
                process::exit(2);
            },
        }
    }

    Ok(())
}
