mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    println!("{:#?}", cli::CommandLineArgs::from_args());

    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    //Unpack the journal file.
    //let journal_file = journal_file.expect("failed to find journal file");
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        //.expect("failed to find json");
        .ok_or(anyhow!("Failed to find json"))?;

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::compare_task(journal_file, position),
    }?;

    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("rusty-journal.json");
        path
    })
}
