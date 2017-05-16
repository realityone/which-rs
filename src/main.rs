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
    println!("{:?}", options);
    0i32
}

fn main() {
    std::process::exit(uumain(std::env::args().collect()));
}
