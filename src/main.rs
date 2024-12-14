mod specification_pdf;
mod subject;
mod test_pdf;

use pdf_extract::extract_text;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use subject::{extract_subject_from_filename, Subject};
use test_pdf::TestPDF;

const DISALLOWED_CHARACTERS: [char; 55] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '–', '/', ' ',
];

const VALID_CHARACTERS: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const TESTS_DIRECTORY: &str = "tests";

fn get_codes(filename: &String) -> Vec<String> {
    let text: String = extract_text(filename).unwrap();

    let codes: Vec<String> = text
        .lines()
        .map(|line: &str| line.trim().to_string())
        .filter(|line: &String| line.contains(VALID_CHARACTERS))
        .filter(|line: &String| !line.contains(DISALLOWED_CHARACTERS))
        .filter(|line: &String| line.len() == 5 || line.len() == 7)
        .filter(|line: &String| line.matches('.').count() == 2 || line.matches('.').count() == 3)
        .collect();

    codes
}

fn get_code_counts(codes: &[String]) -> Vec<(String, i32)> {
    let counts: HashMap<String, i32> = codes.iter().fold(
        HashMap::new(),
        |mut accumulator: HashMap<String, i32>, pattern: &String| {
            *accumulator.entry(pattern.to_string()).or_insert(0) += 1;
            accumulator
        },
    );

    let mut sorted_counts: Vec<_> = counts.into_iter().collect();
    sorted_counts.sort_by(|a: &(String, i32), b: &(String, i32)| b.1.cmp(&a.1));
    sorted_counts
}

fn main() {
    // TODO: do this for each subject individually
    let filenames: Vec<PathBuf> = fs::read_dir(TESTS_DIRECTORY)
        .unwrap()
        .map(|file: Result<fs::DirEntry, std::io::Error>| file.unwrap().path())
        .filter(|path: &PathBuf| path.extension() == Some(OsStr::new("pdf")))
        .collect();

    // TODO: for each subject -> get full list of codes from specifications
    // TODO: for each subject -> get list of codes that are missing from the PDFs

    let mut test_pdfs: Vec<TestPDF> = vec![];
    filenames.iter().for_each(|filename: &PathBuf| {
        println!("Parsing data from {:?}", filename);
        let filename: String = filename.to_string_lossy().to_string();
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
