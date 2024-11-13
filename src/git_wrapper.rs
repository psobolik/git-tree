use std::path::PathBuf;

use self::{
    git_object::GitObject, git_object_type::GitObjectType, git_wrapper_error::GitWrapperError,
};

mod git_command;
mod git_object;
mod git_object_type;
mod git_wrapper_error;

pub struct GitWrapper;

impl GitWrapper {
    pub fn ls_tree_objects(&self, path: &PathBuf) -> Result<Vec<GitObject>, GitWrapperError> {
        match self.ls_tree(path) {
            Ok(ls_tree) => Ok(parse_tree(ls_tree)),
            Err(error) => Err(error),
        }
    }

    pub fn ls_tree(&self, path: &PathBuf) -> Result<String, GitWrapperError> {
        git_command::ls_tree(path)
    }

    pub fn top_level(&self, path: &PathBuf) -> Result<String, GitWrapperError> {
        match git_command::top_level(path) {
            Ok(top_level) => Ok(String::from(top_level.trim_end())),
            Err(error) => Err(error),
        }
    }

    pub fn is_inside_work_tree(&self, path: &PathBuf) -> Result<bool, GitWrapperError> {
        git_command::is_inside_work_tree(path)
    }
}

fn parse_tree(tree_string: String) -> Vec<GitObject> {
    let regex = regex::Regex::new(r"^(?<mode>\d+)\s+(?<type>\S+)\s+(?<name>\S+)\s+(?<size>\S+)\s+(?<path>\S+)$").unwrap();
    let mut result = vec![];
    let lines = tree_string.lines();
    for line in lines {
        if let Some(fields) = regex.captures(&line) {
            result.push(GitObject::new(
                u32::from_str_radix(&fields["mode"], 8).unwrap_or_default(),
                match &fields["type"] {
                    "commit" => Some(GitObjectType::Commit),
                    "blob" => Some(GitObjectType::Blob),
                    "tree" => Some(GitObjectType::Tree),
                    &_ => None, // This should never happen
                },
                String::from(&fields["name"]),
                match fields["size"].parse::<usize>() {
                    Ok(size) => Some(size),
                    _ => None, // "tree" objects have no size
                },
                String::from(&fields["path"]),
            ));
        }
    }
    result
}
