use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new task
    New(NewTask),

    /// Complete a task by ID
    Complete(CompleteTask),

    /// Delete a task by ID
    Delete(DeleteTask),

    /// Display all tasks
    Display,
}

#[derive(Args)]
struct NewTask {
    /// Task Title
    title: String,

    /// A Short Description
    #[arg(short, long)]
    description: Option<String>,
}

#[derive(Args)]
struct CompleteTask {
    /// Task ID
    id: i32,
}

#[derive(Args)]
struct DeleteTask {
    /// Task ID
    id: i32,
}
