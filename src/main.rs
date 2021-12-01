pub mod commands;

use commands::{day0::Day0, CommandImpl, DynError};
use commands::{day1a::Day1a};
use commands::{day1b::Day1b};
use enum_dispatch::enum_dispatch;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Day0(Day0),
    Day1a(Day1a),
    Day1b(Day1b),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}
