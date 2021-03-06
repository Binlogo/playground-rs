mod cli;
mod tasks;
use std::path::PathBuf;

use structopt::StructOpt;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use tasks::*;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { text } => add_task(journal_file, Task::new(text)),
        Done { position } => complete_task(journal_file, position),
        List => list_tasks(journal_file),
    }?;

    Ok(())
}

fn default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
