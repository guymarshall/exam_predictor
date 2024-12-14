mod codes;
mod specification_pdf;
mod subject;
mod test_pdf;

use codes::{get_code_counts, get_codes};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use subject::{extract_subject_from_filename, Subject};
use test_pdf::TestPDF;

fn main() {
    // TODO: do this for each subject individually
    let filenames: Vec<String> = fs::read_dir("tests")
        .unwrap()
        .map(|file: Result<fs::DirEntry, std::io::Error>| file.unwrap().path())
        .filter(|path: &PathBuf| path.extension() == Some(OsStr::new("pdf")))
        .map(|path: PathBuf| path.to_string_lossy().to_string())
        .collect();

    // TODO: for each subject -> get full list of codes from specifications
    // TODO: for each subject -> get list of codes that are missing from the PDFs

    let mut test_pdfs: Vec<TestPDF> = vec![];
    filenames.into_iter().for_each(|filename: String| {
        println!("Parsing data from {:?}", filename);
        let subject: Subject = extract_subject_from_filename(&filename);
        let codes: Vec<String> = get_codes(&filename);
        let current_code_counts: Vec<(String, i32)> = get_code_counts(&codes);
        let test_pdf: TestPDF = TestPDF::new(filename, subject, codes, current_code_counts);
        test_pdfs.push(test_pdf);
    });

    test_pdfs
        .iter()
        .for_each(|test_pdf: &TestPDF| println!("{}", test_pdf));

    // TODO: write list of codes in descending order to file for each subject e.g. biology.txt
}
