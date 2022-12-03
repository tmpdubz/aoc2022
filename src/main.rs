// problem 1 import module
mod problem1;
use problem1::problem1 as p1;

// cli arg parser == clap
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which solution to run
    #[arg(long)]
    problem: i8,

    #[arg(short, long)]
    part: i8
}

// run from cli
fn main() {
    let args = Args::parse();
    let pair = (args.problem, args.part);
    match pair {
        (1,0) => p1::print_sample(),
        (1,1) => p1::print_part1(),
        (1,2) => p1::print_part2(),
        _ => println!("Problem {} Part {}, hasn't been implemented yet.", args.problem, args.part),
    }
}
