use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new task
    New(NewTask),
}

#[derive(Args)]
struct NewTask{
    /// Task Title
    #[arg(short, long)]
    title: String,

    /// A Short Description
    #[arg(short, long)]
    description: Option<String>,
}