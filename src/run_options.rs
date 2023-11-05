use std::path::{PathBuf, Path};

use clap::Parser;

use self::{prefix_set::PrefixSet, clargs::Clargs};

mod clargs;
pub mod prefix_set;

pub struct RunOptions {
    clargs: Clargs,
    prefix_set: PrefixSet,
    folder_absolute: PathBuf,
}

impl RunOptions {
    pub fn new() -> RunOptions {
        let clargs = Clargs::parse();
        let prefix_set = if clargs.ascii {
            PrefixSet {
                parent_prefix: r"|   ".to_string(),
                no_parent_prefix: r"    ".to_string(),
                entry_prefix: r"|-- ".to_string(),
                last_entry_prefix: r"`-- ".to_string(),                
            }
        } else {
            PrefixSet {
                parent_prefix: r"│   ".to_string(),
                no_parent_prefix: r"    ".to_string(),
                entry_prefix: r"├── ".to_string(),
                last_entry_prefix: r"└── ".to_string(),
            }
        };
        let folder_path = Path::new(clargs.folder.as_str()).to_path_buf();
        let folder_absolute = if folder_path.is_absolute() {
            folder_path
        } else {
            match RunOptions::absolute_path(&folder_path) {
                Ok(f) => f,
                _  => folder_path, // We'll validate the path later...
            }
        };
        RunOptions { clargs, prefix_set, folder_absolute }
    }

    pub fn requested_path(&self) -> &String { &self.clargs.folder }
    pub fn prefix_set(&self) -> &PrefixSet { &self.prefix_set }
    pub fn initial_path(&self) -> &PathBuf { &self.folder_absolute }

    fn absolute_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
        let path = path.as_ref();

        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        match absolute_path.canonicalize() {
            Ok(path) => Ok(path),
            Err(_) => Ok(absolute_path)
        }
    }
}
