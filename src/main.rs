pub mod repl;
pub mod expr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Mathematical Proof System")]
pub struct Args {
}

fn main() {
    let args = Args::parse();

    repl::start_repl(args);
}
