mod cli;
mod tasks;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};
use tasks::*;

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    let journal_file = journal_file.unwrap_or("rusty-journal.json".into());

    match action {
        Add { text } => add_task(journal_file, Task::new(text)),
        Done { position } => complete_task(journal_file, position),
        List => list_tasks(journal_file),
    }
    .expect("Failed to perform action");
}
