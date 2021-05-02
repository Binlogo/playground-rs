mod cli;
mod tasks;
use structopt::StructOpt;

fn main() {
    let args = cli::CommandLineArgs::from_args();
    println!("{:#?}", args);
}
