mod cli;
mod database;
use clap::Parser;

fn main() {
    let conn = database::init();

    let cli = cli::Cli::parse();
    let command = &cli.command;

    match command {
        cli::Commands::New(task) => database::insert_task(&conn, task),
        cli::Commands::Complete(task) => database::complete_task(&conn, task.id),
        cli::Commands::Delete(task) => database::delete_task(&conn, task.id),
        cli::Commands::Display => todo!(),
    }
}
