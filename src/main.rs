#[allow(unused_results)]
#[warn(unused_imports)]
#[warn(unused_variables)]
#[warn(dead_code)]

mod git;
mod cli;

fn main() {
    // let args = Cli::parse();
    // println!("{}", String::from(args.pattern));
    git::Diff::get();
}
