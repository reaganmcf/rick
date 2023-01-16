use clap::{Parser, Subcommand};

mod docs;
mod fzf;
mod todo;

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
    Add {
        #[arg(required = true)]
        title: String,
        #[arg(required = false)]
        desc: Option<String>,
    },

    /// List all your todos
    List,

    /// Monitor
    Monitor
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
            TodoCommands::List => todo::list(),
            TodoCommands::Monitor => todo::monitor(),
        }
    }
}
