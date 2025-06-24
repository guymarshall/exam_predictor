use std::{ffi::OsStr, fs, path::PathBuf};

pub(crate) fn get_filenames(path: &str) -> Vec<String> {
    let filenames: Vec<String> = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|file: Result<fs::DirEntry, std::io::Error>| file.expect("Failed to get file").path())
        .filter(|path: &PathBuf| {
            path.extension()
                .expect("Failed to get extension")
                .to_ascii_lowercase()
                == OsStr::new("pdf")
        })
        .map(|path: PathBuf| path.to_string_lossy().to_string())
        .collect();
    filenames
}

pub(crate) fn get_subject_specific_filenames(path: &str, subject_code: i32) -> Vec<String> {
    let subject_filenames: Vec<String> = get_filenames(path);

    subject_filenames
        .into_iter()
        .filter(|filename: &String| filename.contains(&subject_code.to_string()))
        .collect()
}
