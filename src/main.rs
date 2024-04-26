use lcoviz::operations::run;
use std::env::args;

fn main() {
    let args = args().skip(1).collect::<Vec<String>>();
    run(args)
}
