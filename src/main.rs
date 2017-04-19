#[macro_use]
extern crate uucore;

#[allow(dead_code)]
static SYNTAX: &'static str = "[options] <pid> [...]"; 
static SUMMARY: &'static str = ""; 
static LONG_HELP: &'static str = ""; 

#[derive(Clone)]
struct EchoOptions {
    newline: bool,
    escape: bool,
}

fn main() {
    let args = vec!["-h".to_string(), "123".to_string(), "456".to_string()];
    let matches = new_coreopts!(SYNTAX, SUMMARY, LONG_HELP)
        .optopt("s", "signal", "specify the <signal> to be sent", "SIGNAL")
        .optflagopt("l", "list", "list all signal names, or convert one to a name", "LIST")
        .optflag("L", "table", "list all signal names in a nice table")
        .parse(args);
}
