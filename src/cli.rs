use clap::{Parser, arg};

#[derive(Parser)]
pub struct Cli {
    // Optional pattern
    #[arg(short = 'p', long = "pattern", default_value ="")]
    pub pattern: String,

    #[arg(long)]
    pub staged: bool,
}

/*
let re = Regex::new(&args.pattern).unwrap();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        if re.is_match(&line) {
            println!("{}", line);
        }
    }
*/
