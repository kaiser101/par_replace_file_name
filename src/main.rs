use rayon::prelude::*;
use regex::Regex;
use std::fs;
use walkdir::{DirEntry, WalkDir};

const FILE_ROOT: &str = "E:/Administrator/Downloads/Code samples/Node/torrent_client_js/torrents/";
// const FILE_ROOT: &str = "K:/Movies/New/";

fn has_name(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s: &str| s.contains("_Yurievij_"))
        .unwrap_or(false)
}

fn replace_file_name(orig: &str) -> String {
    let re = Regex::new(r".+?_Yurievij_").unwrap();

    let new_name = String::from(FILE_ROOT) + re.replace_all(orig, "").as_ref();

    let _ = fs::rename(orig, &new_name);

    new_name
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _walker = WalkDir::new(FILE_ROOT)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|e: &DirEntry| has_name(&e))
        .map(|e: DirEntry| replace_file_name(&e.path().to_string_lossy()))
        .for_each(|e: String| println!("{}", e));

    Ok(())
}
