mod cli;
mod database;
use clap::Parser;

fn main() {
    let conn = database::init();

    let cli = cli::Cli::parse();
    let command = &cli.command;

    match command {
        cli::Commands::New(task) => database::insert_task(&conn, task),
        cli::Commands::Complete(_task) => todo!(),
        cli::Commands::Delete(_task) => todo!(),
        cli::Commands::Display => todo!(),
    }
}
