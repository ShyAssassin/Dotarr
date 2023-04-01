mod cli;
mod config;
use clap::{Parser, CommandFactory};

fn main() {
    let args = cli::Cli::parse();

    match args.command {
        cli::CliSubcommand::Init { force } => {
            println!("Initializing dotfile repository... force: {force:?}");
        }

        cli::CliSubcommand::Add { file } => {
            println!("Adding dotfile {file:?}...");
        }

        cli::CliSubcommand::Remove { file } => {
            println!("Removing dotfile {file:?}...");
        }

        cli::CliSubcommand::Edit { path, editor } => {
            println!("Editing dotfile {path:?} with editor {editor:?}...");
        }

        cli::CliSubcommand::List => {
            println!("Listing dotfiles...");
        }

        cli::CliSubcommand::Generate { generator } => {
            let mut cmd = cli::Cli::command();
            println!("Generating completion file for {generator:?}...");
            cli::print_completions(generator, &mut cmd);
        }
    }
}
