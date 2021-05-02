use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add a task
    Add {
        /// Task description
        #[structopt()]
        text: String,
    },
    /// Complete a task, remove by position
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty journal", about = "A command line todo app")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
