use std::env;
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;

#[macro_use]
extern crate uucore;
extern crate which;

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
    let files: Vec<&str> = matches.free.iter()
        .filter(|f| f.len() > 0)
        .map(|f| f.as_ref())
        .collect();

    let paths = match env::var_os("PATH") {
        Some(path) => env::split_paths(&path).collect::<Vec<_>>(),
        None => vec![],
    };
    let paths: Vec<&Path> = paths.iter().map(|pb| pb.as_path()).collect();

    let mut find_path = HashMap::new();
    let all_matched = crash_if_err!(1, which::which(
            files.as_slice(),
            &paths,
            options.all_matches,
            Some(&mut find_path)));

    if !options.silence {
        for (_, targets) in &find_path {
            for p in targets {
                println!("{}", p.to_string_lossy())
            }
        }
    }

    match all_matched {
        true => 0,
        false => 1,
    }
}

fn main() {
    std::process::exit(uumain(std::env::args().collect()));
}
