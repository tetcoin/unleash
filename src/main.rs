use std::error::Error;
use structopt::StructOpt;
mod cli;
mod commands;
mod util;
mod matcher;

use cli::Opt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut argv = Vec::new();
    let mut args = std::env::args();
    argv.extend(args.next());
    if let Some(h) = args.next() {
        if h != "unleash" {
            argv.push(h)
        }
    }
    argv.extend(args);
    cli::run(Opt::from_iter(argv))
}
