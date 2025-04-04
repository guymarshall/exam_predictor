use std::fs::File;
use std::io::Read;
use std::{ffi::OsStr, fs, path::PathBuf};

pub(crate) fn get_filenames(path: &str, extension: &str) -> Vec<String> {
    let filenames: Vec<String> = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|file: Result<fs::DirEntry, std::io::Error>| file.expect("Failed to get file").path())
        .filter(|path: &PathBuf| {
            path.extension()
                .expect("Failed to get extension")
                .to_ascii_lowercase()
                == OsStr::new(extension)
        })
        .map(|path: PathBuf| path.to_string_lossy().to_string())
        .collect();
    filenames
}

pub(crate) fn get_subject_specific_filenames(
    path: &str,
    extension: &str,
    subject_code: i32,
) -> Vec<String> {
    let subject_filenames: Vec<String> = get_filenames(path, extension);

    subject_filenames
        .into_iter()
        .filter(|filename: &String| filename.contains(&subject_code.to_string()))
        .collect()
}

pub(crate) fn extract_text_from_txt(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    contents
}
