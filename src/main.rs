use clap::{Parser, Subcommand};

mod docs;
mod todo;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Open the documentation for a specific library, framework, or language
    #[command(subcommand)]
    Docs(DocsCommands),

    /// Manage your todos
    #[command(subcommand)]
    Todo(TodoCommands),
}

#[derive(Subcommand, Debug)]
enum DocsCommands {
    /// Search tailwind docs
    Tw,

    /// Search mdn docs
    Mdn,
}

#[derive(Subcommand, Debug)]
enum TodoCommands {
    /// Add a new todo
    Add {
        #[arg(required = true)]
        title: String,
        #[arg(required = false)]
        desc: Option<String>,
    },
    /// Edit a todo by id
    Edit {
        #[arg(required = true)]
        id: usize,
        #[arg(short = 't', long = "title", required = false)]
        new_title: Option<String>,
        #[arg(short = 'd', long = "desc", required = false)]
        new_desc: Option<String>,
    },
    /// Delete a todo by id
    Delete {
        #[arg(required = true)]
        id: usize,
    },
    /// List all of your current todos
    List,
    /// Monitor your todos, refreshing periodically (default is 5 seconds)
    Monitor {
        #[arg(short = 't', long = "timeout", required = false, help = "seconds to wait before refreshing")]
        timeout: Option<u64>
    }
}

fn main() {
    let args = Args::parse();

    //println!("{:#?}", args.command);
    match args.command {
        Commands::Docs(inner) => match inner {
            DocsCommands::Tw => docs::tailwind::search_and_open(),
            DocsCommands::Mdn => docs::mdn::search_and_open(),
        },
        Commands::Todo(inner) => match inner {
            TodoCommands::Add { title, desc } => todo::add(title, desc),
            TodoCommands::Edit {
                id,
                new_title,
                new_desc,
            } => todo::edit(id, new_title, new_desc),
            TodoCommands::List => todo::list(),
            TodoCommands::Monitor { timeout } => todo::monitor(timeout.unwrap_or(5)),
            TodoCommands::Delete { id } => todo::delete(id),
        },
    }
}
