use std::fs;
use std::io;
use std::env;
use std::path::{Path, PathBuf};

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>, verbose: bool ) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()), verbose)?;
        } else {
            if verbose {
                println!("Copying {:?} --> {:?}", entry.path(), destination.as_ref().join(entry.file_name()));
            }
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn get_dotfiles_dir() -> PathBuf {
    let home_dir = env::var("HOME").unwrap();
    let dotfiles_dir = Path::new(&home_dir).join(".dotfiles");
    return dotfiles_dir;
}

// This function is used to convert a relative path to an absolute path
// Example: .config/nvim/init.vim -> /home/user/.config/nvim/init.vim
pub fn get_full_path(file: &PathBuf) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let full_path = current_dir.join(&file);
    return full_path;
}

// This function is used to convert an absolute path to a full path relative within the dotfiles directory
// Example: ~/.config/nvim/init.vim -> ~/.dotfiles/.config/nvim/init.vim
pub fn get_dest_path(file: &PathBuf) -> PathBuf {
    let dotfile_dir = get_dotfiles_dir();
    let full_path = get_full_path(&file);
    let dest_path = dotfile_dir.join(full_path.strip_prefix(&dotfile_dir).unwrap());
    return dest_path;
}

// This function is used to convert an absolute path to a relative path relative to the dotfiles directory
// Example: ~/.config/nvim/init.vim -> .config/nvim/init.vim
pub fn get_relative_dest_path(file: &PathBuf) -> PathBuf {
    let dotfile_dir = get_dotfiles_dir();
    let full_path = get_full_path(&file);
    let dest_path = full_path.strip_prefix(&dotfile_dir).unwrap();
    return dest_path.to_path_buf();
}