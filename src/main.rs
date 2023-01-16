use clap::{Parser, Subcommand};

mod docs;
mod fzf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Open the documentation for a specific library, framework, or language
    #[command(subcommand)]
    Docs(DocsCommands)
}

#[derive(Subcommand, Debug)]
enum DocsCommands {
    /// Search tailwind docs
    Tw,

    /// Search mdn docs
    Mdn
}

fn main() {
    let args = Args::parse();

    //println!("{:#?}", args.command);
    match args.command {
        Commands::Docs(inner) => match inner {
            DocsCommands::Tw => docs::tailwind::search_and_open(),
            DocsCommands::Mdn => docs::mdn::search_and_open(),
        }
    }
}
