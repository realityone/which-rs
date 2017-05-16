use std::env;
use std::io::Write;
use std::os::unix::prelude::*;

#[macro_use]
extern crate uucore;

#[allow(dead_code)]
static SYNTAX: &'static str = "[options] <program> [...]";
static SUMMARY: &'static str = "which -- locate a program file in the user's path";
static LONG_HELP: &'static str = "
 The which utility takes a list of command names and searches the path for each executable \
     file that would be run
 had these commands actually been invoked.
";

#[derive(Debug, Clone)]
struct WhichOptions {
    all_matches: bool,
    silence: bool,
}

pub fn uumain(args: Vec<String>) -> i32 {
    let matches = new_coreopts!(SYNTAX, SUMMARY, LONG_HELP)
        .optflag("a",
                 "all-matches",
                 "list all instances of executables found (instead of just the first one of each)")
        .optflag("s",
                 "silence",
                 "no output, just return 0 if any of the executables are found, or 1 if none are \
                  found")
        .parse(args);
    let options = WhichOptions {
        all_matches: matches.opt_present("all-matches"),
        silence: matches.opt_present("silence"),
    };
    let files = matches.free;

    let mut all_matched = true;
    let paths = match env::var_os("PATH") {
        Some(path) => env::split_paths(&path).collect::<Vec<_>>(),
        None => vec![],
    };
    for f in &files {
        let mut matched = false;
        for p in &paths {
            let mut target = p.clone();
            target.push(f);

            // file not exists
            if !target.exists() {
                continue;
            }

            let metadata =
                target.metadata().map_err(|_| crash!(1, "read metadata failed")).unwrap();
            if metadata.mode() & 0o111 == 0 {
                // Not an executable file
                continue;
            }

            // Find an executable file
            matched = true;
            if !options.silence {
                println!("{}", target.to_string_lossy());
                if !options.all_matches {
                    break;
                }
            }
        }

        all_matched &= matched;
    }

    match all_matched {
        true => 0,
        false => 1,
    }
}

fn main() {
    std::process::exit(uumain(std::env::args().collect()));
}
