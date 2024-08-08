//! Clean up the generated documentation



use std::path::{Path, PathBuf};

use anyhow::Result;



fn main() {
    let docs_path = PathBuf::from(std::env::args().nth(1).expect("invalid args"));
    let html_files = scan_for_ext(&docs_path, "html").expect("error finding html files");

    for html_file in html_files {
        let mut s = std::fs::read_to_string(&html_file).expect("html file not parsable as string");
        s = s.replace("::", "/");
        std::fs::write(&html_file, s).expect("html file not writable");
    }
}

fn scan_for_ext(path: &Path, ext: &str) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];

    for e in std::fs::read_dir(path)? {
        let entry_path = e?.path();
        if entry_path.is_dir() {
            paths.extend(scan_for_ext(&entry_path, ext)?);
        } else if entry_path.extension().is_some_and(|e| e == ext) {
            paths.push(entry_path);
        }
    }

    Ok(paths)
}
