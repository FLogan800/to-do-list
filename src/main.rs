mod cli;
mod database;
mod task;
use clap::Parser;
use task::Task;

fn main() {
    let conn = database::init();

    let cli = cli::Cli::parse();
    let command = &cli.command;

    match command {
        cli::Commands::New(task) => new_task(&conn, task),
        cli::Commands::Complete(_task) => todo!(),
        cli::Commands::Delete(_task) => todo!(),
        cli::Commands::Display => todo!(),
    }
}

fn new_task(conn: &rusqlite::Connection, task: &cli::NewTask) {
    let id = database::get_max_id(conn) + 1;

    let task = task::Task {
        id: id,
        title: (*task).title.clone(),
        description: task.description.clone(),
        complete: false,
    };

    database::insert_task(conn, task);
}
