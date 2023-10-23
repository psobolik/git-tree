mod git_wrapper;
mod run_options;

use std::path::PathBuf;
use std::process::ExitCode;

use git_wrapper::GitWrapper;
use run_options::RunOptions;

use crate::run_options::prefix_set::PrefixSet;

fn main() -> ExitCode {
    let run_options = &RunOptions::new();

    let git_wrapper = GitWrapper {};

    match git_wrapper.is_inside_work_tree(run_options.initial_path()) {
        Ok(is_inside_work_tree) => {
            if !is_inside_work_tree {
                eprintln!(
                    "Error: {:?} is not inside a git work tree",
                    run_options.requested_path()
                );
                return ExitCode::from(1);
            }
        }
        Err(error) => {
            eprintln!(
                "Error: couldn't access {:?}: {}",
                run_options.requested_path(),
                error.message()
            );
            return ExitCode::from(2);
        }
    };

    let top_level = match git_wrapper.top_level(run_options.initial_path()) {
        Ok(top_level) => {
            if PathBuf::from(&top_level).exists() {
                PathBuf::from(top_level)
            } else {
                eprintln!(
                    "Error: no git top level path for {:?}",
                    run_options.initial_path(),
                );
                return ExitCode::from(3);
            }
        }
        Err(error) => {
            eprint!(
                "Error: couldn't get git top level path for {:?}: {}",
                run_options.initial_path(),
                error.message()
            );
            return ExitCode::from(4);
        }
    };

    println!(r#"Top level: "{}""#, top_level.to_string_lossy());

    let mut flags: Vec<bool> = vec![];
    print_git_tree(&top_level, run_options, &mut flags, &git_wrapper);

    ExitCode::SUCCESS
}

fn print_git_tree(
    path: &PathBuf,
    run_options: &RunOptions,
    flags: &mut Vec<bool>,
    git_wrapper: &GitWrapper,
) {
    match git_wrapper.ls_tree_objects(path) {
        Ok(ls_tree_objects) => {
            let last_index = ls_tree_objects.len() - 1;
            (0..ls_tree_objects.len()).for_each(|index| {
                let lto = &ls_tree_objects[index];
                let is_last_entry = index == last_index;
                let prefix = get_prefix(flags, is_last_entry, run_options.prefix_set());
                if lto.is_tree() {
                    println!("{}{}", prefix, lto.path());
                    flags.push(true);
                    if is_last_entry {
                        *flags.last_mut().unwrap() = false
                    }
                    print_git_tree(
                        &path.join(PathBuf::from(lto.path())),
                        run_options,
                        flags,
                        git_wrapper,
                    );
                    flags.pop();
                } else {
                    println!("{}{}", prefix, lto.path());
                }
            });
        }
        Err(error) => eprintln!(
            "Error: couldn't get git tree objects in {:?}: {}",
            path,
            error.message()
        ),
    };
    fn get_prefix(flags: &Vec<bool>, is_last_entry: bool, prefix_set: &PrefixSet) -> String {
        let mut result = String::new();
        for flag in flags {
            result += if *flag {
                prefix_set.parent_prefix.as_str()
            } else {
                prefix_set.no_parent_prefix.as_str()
            }
        }
        result += if is_last_entry {
            prefix_set.last_entry_prefix.as_str()
        } else {
            prefix_set.entry_prefix.as_str()
        };
        result
    }
}
