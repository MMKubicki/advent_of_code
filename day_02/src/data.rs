use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day_01", about = "Day 1 of Advent of Code.")]
pub struct Opt {
    ///Path to inputfile
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    ///Position of computer to print
    #[structopt(short, long)]
    pub position: usize,

    ///Save memory on every step. Saved to dump.txt
    #[structopt(short, long)]
    pub memory: bool,
}
