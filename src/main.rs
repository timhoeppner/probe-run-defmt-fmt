mod handler;

use std::fs::File;
use std::io::{self, BufReader};
use atty::Stream;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Path to JSON file, this will be ignored if data is available from STDIN.
    #[clap(short, long)]
    file: String,
}

/*
    TODO

    1. output format from env variable
    2. output format from yml config
    3. default output format
    4. with colors (on by default)
 */

fn main() {
    if atty::is(Stream::Stdin) {
        let args = Args::parse();
        let file = File::open(args.file).expect("Unable to open file");
        let stream = BufReader::new(file);
        handler::process(stream);
    } else {
        let stdin = io::stdin();
        let stream = stdin.lock();
        handler::process(stream);
    }
}
