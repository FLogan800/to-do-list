mod cli;
mod database;
mod task;
use clap::Parser;
use task::Task;

fn main() {
    let _conn = database::init();

    let _cli = cli::Cli::parse();
}
