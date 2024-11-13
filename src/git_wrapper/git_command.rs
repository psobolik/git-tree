use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use super::git_wrapper_error::GitWrapperError;

/// Returns the git tree for a path as a string, or an error (probably because
/// the path is invalid)
pub fn ls_tree(path: &PathBuf) -> Result<String, GitWrapperError> {
    git_command(
        path,
        "ls-tree",
        Vec::from([
            "--long",
            "HEAD",
        ]),
    )
}

/// Returns the top-level git folder for a given folder, or an error (probably
/// because the path is invalid)
pub fn top_level(path: &PathBuf) -> Result<String, GitWrapperError> {
    git_command(path, "rev-parse", vec!["--show-toplevel"])
}

/// Returns a flag indicating whether a path is inside a git work tree, or an 
/// error (probably because the path is invalid)
pub fn is_inside_work_tree(path: &PathBuf) -> Result<bool, GitWrapperError> {
    git_command_status(path, "rev-parse", vec!["--is-inside_work-tree"])
}

/// Returns a git command in a vector with its arguments
fn git_args<'a>(command: &'a str, args: Vec<&'a str>) -> Vec<&'a str> {
    let mut git_args = vec![command];
    for arg in args {
        git_args.push(arg);
    }
    git_args
}

/// Runs a git command and returns its output
fn git_command(path: &PathBuf, command: &str, args: Vec<&str>) -> Result<String, GitWrapperError> {
    match Command::new("git")
        .current_dir(path)
        .args(git_args(command, args))
        .stdout(Stdio::piped())
        .output()
    {
        Ok(git_command) => Ok(unsafe { String::from_utf8_unchecked(git_command.stdout) }),
        Err(error) => Err(GitWrapperError::new(error.to_string())),
    }
}

/// Runs a git command and return its status
fn git_command_status(
    path: &PathBuf,
    command: &str,
    args: Vec<&str>,
) -> Result<bool, GitWrapperError> {
    match Command::new("git")
        .current_dir(path)
        .args(git_args(command, args))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(git_command) => Ok(git_command.success()),
        Err(error) => Err(GitWrapperError::new(error.to_string())),
    }
}
