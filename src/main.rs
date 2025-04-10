mod cli;
mod database;
mod task;

use std::cmp::Reverse;

use clap::Parser;
use cli::DisplayTasks;

fn main() {
    let conn = database::init();

    let cli = cli::Cli::parse();
    let command = &cli.command;

    match command {
        cli::Commands::New(task) => database::insert_task(&conn, task),
        cli::Commands::Complete(task) => database::complete_task(&conn, task.id),
        cli::Commands::Incomplete(task) => database::uncomplete_task(&conn, task.id),
        cli::Commands::Delete(task) => database::delete_task(&conn, task.id),
        cli::Commands::Display(sorting) => display_tasks(&conn, sorting),
    }
}

fn display_tasks(conn: &rusqlite::Connection, sorting: &DisplayTasks) {
    let mut tasks = database::fetch_tasks(conn);

    if sorting.complete {
        tasks.retain(|task| task.is_complete);
    } else if sorting.incomplete {
        tasks.retain(|task| !task.is_complete);
    }

    if sorting.priority && sorting.due_date {
        tasks.sort_by_key(|task| (Reverse(task.priority), task.due_date));
    } else if sorting.priority {
        tasks.sort_by_key(|task| (Reverse(task.priority)));
    } else if sorting.due_date {
        tasks.sort_by_key(|task| (task.due_date));
    }

    for task in tasks {
        println!("{}", task);
    }
}
