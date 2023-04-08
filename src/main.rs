mod cli;
mod util;
mod config;
use std::env;
use std::path::{Path};
use std::fs;
use std::fs::{copy, create_dir_all, remove_file};
use clap::{Parser, CommandFactory};
use git2::Repository;
use regex::Regex;

fn main() {
    let home_dir = env::var("HOME").unwrap();
    let dotfiles_dir = Path::new(&home_dir).join(".dotfiles");
    let current_dir = env::current_dir().unwrap();
    let args = cli::Cli::parse();


    match args.command {
        cli::CliSubcommand::Init { repository, force } => {
            match repository {
                Some(mut repository) => {
                    let re = Regex::new(r"^(http://|https://|git@|git://|ssh://|git\+ssh://)([^/]+)(/.*)$").unwrap();
                    if !re.is_match(&repository) {
                        // TODO: search user's github for for multiple matches
                        repository = format!("https://github.com/{}/dotfiles", repository);
                    }
                    println!("Cloning repository {repository:?} to {dotfiles_dir:?}...");
                    if dotfiles_dir.exists() && !force {
                        println!("Repository already exists, use --force to overwrite!");
                    } else if dotfiles_dir.exists() && force {
                        println!("Overwriting existing repository...");
                        fs::remove_dir_all(&dotfiles_dir).unwrap();
                        let _repo = match Repository::clone(&repository, &dotfiles_dir) {
                            Ok(repo) => repo,
                            Err(e) => panic!("failed to clone: {e:?}"),
                        };
                    } else {
                        let _repo = match Repository::clone(&repository, &dotfiles_dir) {
                            Ok(repo) => repo,
                            Err(e) => panic!("failed to clone: {e:?}"),
                        };
                    }
                }

                None => {
                    println!("No repository specified, creating new repository at {dotfiles_dir:?}...");
                    if dotfiles_dir.exists() && !force {
                        println!("Repository already exists, use --force to overwrite!");
                    } else if dotfiles_dir.exists() && force {
                        println!("Overwriting existing repository...");
                        fs::remove_dir_all(&dotfiles_dir).unwrap();
                        create_dir_all(&dotfiles_dir).unwrap();
                        Repository::init(&dotfiles_dir).unwrap();
                    } else {
                        create_dir_all(&dotfiles_dir).unwrap();
                        Repository::init(&dotfiles_dir).unwrap();
                    }

                }
            }
        }

        cli::CliSubcommand::Add { file } => {
            // create full path to file / dir
            let full_path = current_dir.join(&file);
            // get file path relative to home dir
            let dest_path = full_path.strip_prefix(&home_dir).unwrap();
            println!("Adding {full_path:?} to dotfiles repository as {dest_path:?}...");
            if full_path.is_file() {
                // create parent dir if it doesn't exist
                let parent_dir = dotfiles_dir.join(dest_path.parent().unwrap());
                create_dir_all(parent_dir).unwrap();
                // copy file to dotfiles dir
                copy(&full_path, dotfiles_dir.join(dest_path)).unwrap();
            } 
            else if full_path.is_dir() {
                // get file path relative to home dir
                util::copy_recursively(&full_path,  dotfiles_dir.join(dest_path)).expect("Failed to copy directory");
            }
            else {
                println!("Cannot find file or directory!")
            }
        }

        cli::CliSubcommand::Remove { file } => {
            // create full path to file / dir
            let full_path = current_dir.join(&file);
            // get file path relative to home dir
            let dest_path = full_path.strip_prefix(&home_dir).unwrap();
            println!("Removing {full_path:?} from dotfiles repository as {dest_path:?}...");
            if full_path.is_file() {
                remove_file(dotfiles_dir.join(dest_path)).expect("Failed to remove file");
            } else if full_path.is_dir() {
                fs::remove_dir_all(dotfiles_dir.join(dest_path)).expect("Failed to remove directory");
            } else {
                panic!("Cannot find file or directory!")
            }
        }

        cli::CliSubcommand::Edit { path, editor } => {
            println!("Editing dotfile {path:?} with editor {editor:?}...");
        }

        cli::CliSubcommand::Diff { file, viewer } => {
            println!("Diffing dotfile {file:?} with {viewer:?}...");
        }

        cli::CliSubcommand::List => {
            println!("Listing dotfiles...");
        }

        cli::CliSubcommand::Cd => {
            println!("Changing directory to dotfiles repository...");
            // TODO: do this
        }

        cli::CliSubcommand::Generate { generator } => {
            let mut cmd = cli::Cli::command();
            cli::print_completions(generator, &mut cmd);
        }
        
        _ => {
            println!("Not implemented yet!");
        }
    }
}
