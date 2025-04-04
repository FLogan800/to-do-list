mod cli;
mod database;
mod task;
use clap::Parser;

fn main() {
    let conn = database::init();

    let cli = cli::Cli::parse();
    let command = &cli.command;

    match command {
        cli::Commands::New(task) => database::insert_task(&conn, task),
        cli::Commands::Complete(task) => database::complete_task(&conn, task.id),
        cli::Commands::Delete(task) => database::delete_task(&conn, task.id),
        cli::Commands::Display => display_tasks(&conn),
    }
}

fn display_tasks(conn: &rusqlite::Connection) {
    let tasks = database::fetch_tasks(conn);

    for task in tasks {
        println!("{}", task);
    }
}
