use rayon::prelude::*;
use regex::Regex;
use std::fs;
use walkdir::{DirEntry, WalkDir};

const FILE_ROOT: &str = "E:/Administrator/Downloads/Code samples/Node/torrent_client_js/torrents/";

fn has_name(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.contains("_Yurievij_"))
}

fn replace_file_name(entry: &DirEntry, pattern: &Regex) -> Option<String> {
    let path = entry.path();
    let file_name = path.file_name()?.to_str()?;

    // Apply regex ONLY to the filename, not the absolute path string
    let new_file_name = pattern.replace_all(file_name, "");

    // Construct the new path safely
    // (Uses `with_file_name` to rename in the same directory.
    // If you explicitly wanted to move all files to FILE_ROOT, use `Path::new(FILE_ROOT).join(new_file_name.as_ref())` instead)
    let new_path = path.with_file_name(new_file_name.as_ref());

    match fs::rename(path, &new_path) {
        Ok(_) => Some(new_path.to_string_lossy().into_owned()),
        Err(e) => {
            eprintln!("Failed to rename {:?}: {}", path, e);
            None
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pattern = Regex::new(r".+?_Yurievij_")?;

    WalkDir::new(FILE_ROOT)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok) // Cleaner than |e| e.ok()
        .filter(has_name) // Passing function pointer directly
        .filter_map(|e| replace_file_name(&e, &pattern))
        .for_each(|new_name| println!("{}", new_name));

    Ok(())
}
