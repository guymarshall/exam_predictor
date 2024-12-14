use std::{ffi::OsStr, fs, path::PathBuf};

pub(crate) fn get_pdf_filenames(path: &str) -> Vec<String> {
    let filenames: Vec<String> = fs::read_dir(path)
        .unwrap()
        .map(|file: Result<fs::DirEntry, std::io::Error>| file.unwrap().path())
        .filter(|path: &PathBuf| path.extension() == Some(OsStr::new("pdf")))
        .map(|path: PathBuf| path.to_string_lossy().to_string())
        .collect();
    filenames
}
