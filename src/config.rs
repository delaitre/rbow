use std::{path::PathBuf, str::FromStr};

use crate::rule::Rule;

pub struct ConfigPath {
    starting_directory: PathBuf,
    filename: String,
}

impl ConfigPath {
    pub fn new(starting_directory: &str, filename: &str) -> ConfigPath {
        let canonical = PathBuf::from_str(starting_directory)
            .unwrap()
            .canonicalize()
            .unwrap();
        ConfigPath {
            starting_directory: canonical,
            filename: filename.into(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.starting_directory.ancestors().filter_map(|path| {
            let mut path = path.to_path_buf();
            path.push(&self.filename);
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
    }
}

pub struct Config {
    rules: Vec<Rule>,
}

impl Config {
    pub fn load(paths: ConfigPath) -> Config {
        for path in paths.iter() {
            println!("{}", path.to_str().unwrap());
        }
        Config { rules: vec![] }
    }
}
