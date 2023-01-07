use std::{fs, fmt};
use std::path::{Path, PathBuf};

// we need a structure that holds directories and files
// A directory can hold 0 or more directories and 0 or more files
// a file is just a Path
struct Directory {
    files: Vec<PathBuf>,
    dirs: Vec<Directory>,
}

impl Directory {
    fn new(files: Vec<PathBuf>, dirs: Vec<Directory>) -> Self {
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
            let dir_str = &dir.to_string();
            files_string = files_string + dir_str;
        }
        write!(f, "{}", files_string)
    }
}

fn crawl_dir(dir_path: &Path) -> Directory {
    let paths = fs::read_dir(dir_path)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect::<Vec<_>>();
    let subdir_paths = paths
        .iter()
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    
    let mut subdirs = Vec::new();
    for dir in subdir_paths {
        subdirs.push(crawl_dir(dir));
    }

    let root_dir = Directory::new(paths, subdirs);
    root_dir
}

fn main() {
    let dir = crawl_dir(&PathBuf::from("."));
    println!("{}", dir);
}
