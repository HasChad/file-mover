use rand::prelude::*;
use std::{fs, io};
use walkdir::WalkDir;

const OLD_FOLDER: &str = "./Folders/2020/";
const NEW_FOLDER: &str = "./Folders/choosen/";
const FILE_COUNT: i32 = 24;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut files: Vec<walkdir::DirEntry> = vec![];

    for entry in WalkDir::new(OLD_FOLDER).min_depth(1) {
        files.push(entry.unwrap().clone());
    }

    println!("All files are loaded. Press ENTER to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();

    for _ in 0..FILE_COUNT {
        let index = rng.gen_range(0..files.len());
        let old_file = &files[index];
        let new_file = format!("{}", old_file.path().display());
        let new_file = new_file.replace(OLD_FOLDER, "");

        println!("choosen file = {}", new_file);

        fs::copy(old_file.path(), format!("{}{}", NEW_FOLDER, new_file))?;
        fs::remove_file(old_file.path())?;
        files.remove(index);
    }

    Ok(())
}
