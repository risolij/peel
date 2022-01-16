use crate::lib::Peel;
use structopt::StructOpt;

mod lib;

fn main() {
    let peel = Peel::from_args();

    peel.print_header();
    println!("{}", peel.contents());
}
