//! Clean up the generated documentation



use std::path::{Path, PathBuf};

use anyhow::Result;



fn main() {
    let docs_path = PathBuf::from(std::env::args().nth(1).expect("invalid args"));
    let html_files = scan_for_ext(&docs_path, "html").expect("error finding html files");
    let html_file_count = html_files.len();

    for (index, html_file) in html_files.into_iter().enumerate() {
        println!("Updating `{}`... ({}/{})", html_file.display(), index + 1, html_file_count);
        let mut s = std::fs::read_to_string(&html_file).expect("html file not parsable as string");
        s = s.replace("::", " / ");

        // TEMP: Look for `postdate`s
        // {
        //     for range in find_custom_html_elements(&s, "postdate") {
        //         let element_text = &s[range];
        //         println!("Found custom element `postdate` with text \"{element_text}\".");
        //     }
        // }

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



// /// Current Custom Elements:
// /// 
// /// - <postdate text="..."/>
// fn find_custom_html_elements(text: &str, element: &str) -> Vec<Range<usize>>{
//     let mut findings = vec![];
//     for (start_index, _) in text.match_indices(&format!("<div class=\"{element}\"/>")) {
//         if let Some(end_index) = &text[start_index..].find(&format!("</{element}>")) {
//             findings.push(start_index..(start_index + end_index))
//         }
//     }

//     findings
// }
