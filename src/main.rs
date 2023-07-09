use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // get current working directory
    let cwd = env::current_dir().unwrap();

    // get list of files in cwd
    let files = fs::read_dir(&cwd).unwrap();

    // create a list of file extensions
    let mut extensions: Vec<String> = Vec::new();
    for file in files {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_file() {
                if let Some(extension) = file.path().extension() {
                    extensions.push(extension.to_string_lossy().to_string());
                }
            }
        }
    }

    // remove duplicates
    extensions.sort();
    extensions.dedup();

    // create folders for each extension
    for extension in &extensions {
        let dir = Path::new(&cwd).join(extension);
        if !dir.exists() {
            fs::create_dir(&dir).unwrap();
        }
    }

    // move files to their respective folders
    let files = fs::read_dir(&cwd).unwrap();
    for file in files {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_file() {
                if let Some(extension) = file.path().extension() {
                    let dir = Path::new(&cwd).join(extension);
                    let new_path = dir.join(file.file_name());
                    fs::rename(file.path(), new_path).unwrap();
                }
            }
        }
    }

    // print message
    println!("Files sorted by extension.");
}