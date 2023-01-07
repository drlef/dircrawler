use std::{fs, fmt, path::{Path, PathBuf}};
use clap::{Arg, ArgAction, Command};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DirectoryEntry {
    dir_path: PathBuf,
    dir: Directory,
}

impl DirectoryEntry {
    fn new(dir_path: PathBuf, dir: Directory) -> Self {
        Self {
            dir_path,
            dir,
        }
    }
}

impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dir_str = self.dir_path.display().to_string() + "/\n";

        dir_str = dir_str + &self.dir.to_string();
        write!(f, "{}", dir_str)
    }
}

#[derive(Serialize, Deserialize)]
struct Directory {
    files: Vec<PathBuf>,
    dirs: Vec<DirectoryEntry>,
}

impl Directory {
    fn new(files: Vec<PathBuf>, dirs: Vec<DirectoryEntry>) -> Self {
        Self {
            files,
            dirs,
        }
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut files_string = String::new();
        for file in &self.files {
            let file_str = &file.display().to_string();
            files_string = files_string + file_str + "\n";
        }
        for dir in &self.dirs {
            files_string = files_string + &dir.to_string();
        }
        write!(f, "{}", files_string)
    }
}

fn crawl_dir(dir_path: &Path) -> Directory {
    let paths = fs::read_dir(dir_path);
    let paths = match paths {
        Ok(paths) => paths,
        Err(err) => {
            println!("error happened at: {:?}\n{:?}", dir_path, err);
            return Directory{ files: Vec::new(), dirs: Vec::new() }
        },
    };

    let paths = paths.map(|res| res.unwrap().path())
        .collect::<Vec<_>>();

    let subdir_paths = paths
        .iter()
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    let file_paths = paths
        .iter()
        .filter(|path| path.is_file())
        .map(|path| path.clone())
        .collect::<Vec<_>>();

    let mut subdirs = Vec::new();
    for dir in subdir_paths {
        subdirs.push(DirectoryEntry::new(dir.clone(), crawl_dir(dir)));
    }

    let root_dir = Directory::new(file_paths, subdirs);
    root_dir
}

fn main() {
    let m = Command::new("dircrawler")
        .about("Crawls a directory and all subdirectories")
        .arg(
            Arg::new("yaml")
                .short('y')
                .action(ArgAction::SetTrue)
                .help("Output as yaml")
        )
        .arg(
            Arg::new("directory_path")
                .required(true)
                .action(ArgAction::Set)
        )
        .get_matches();

    let directory_path = m.get_one::<String>("directory_path").map(String::as_str).unwrap();
    let dir = crawl_dir(&PathBuf::from(directory_path));

    let yaml = match m.get_one::<bool>("yaml") {
        Some(a) => *a,
        None => false,
    };

    if yaml {
        println!("{}", serde_yaml::to_string(&dir).unwrap());
    } else {
        println!("{}", dir);
    }
}

