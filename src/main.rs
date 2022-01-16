use crate::lib::Peel;
use structopt::StructOpt;

mod lib;

fn main() {
    let peel = Peel::from_args();

    let peel = peel.print_header().contents();
    println!("{}", peel);
}
