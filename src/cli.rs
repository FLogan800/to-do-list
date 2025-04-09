use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new task
    New(NewTask),

    /// Complete a task by ID
    Complete(CompleteTask),

    /// Mark a task incomplete by ID
    Incomplete(IncompleteTask),

    /// Delete a task by ID
    Delete(DeleteTask),

    /// Display all tasks
    Display(DisplayTasks),
}

#[derive(Args)]
pub struct NewTask {
    /// Task Title
    pub title: String,

    /// A Short Description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Task Priority
    #[arg(short, long, default_value_t = 1)]
    pub priority: i32,

    /// Task Due Date (mm/dd/yyyy)
    #[arg(short = 'u', long)]
    pub due_date: Option<String>,
}

#[derive(Args)]
pub struct CompleteTask {
    /// Task ID
    pub id: i32,
}

#[derive(Args)]
pub struct IncompleteTask {
    /// Task ID
    pub id: i32,
}

#[derive(Args)]
pub struct DeleteTask {
    /// Task ID
    pub id: i32,
}

#[derive(Args)]
pub struct DisplayTasks {}
