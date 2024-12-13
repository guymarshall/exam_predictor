use pdf_extract::extract_text;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

const DISALLOWED_CHARACTERS: [char; 55] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '–', '/', ' ',
];

const VALID_CHARACTERS: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn parse_pdf_data(pdf_path: &PathBuf) {
    let text: String = extract_text(pdf_path).unwrap();

    let decimal_lines: Vec<&str> = text
        .lines()
        .map(|line: &str| line.trim())
        .filter(|line: &&str| line.contains(VALID_CHARACTERS))
        .filter(|line: &&str| !line.contains(DISALLOWED_CHARACTERS))
        .filter(|line: &&str| line.len() == 5 || line.len() == 7)
        .filter(|line: &&str| line.matches('.').count() == 2 || line.matches('.').count() == 3)
        .collect();

    let counts: HashMap<&str, i32> = decimal_lines.iter().fold(
        HashMap::new(),
        |mut accumulator: HashMap<&str, i32>, pattern: &&str| {
            *accumulator.entry(pattern).or_insert(0) += 1;
            accumulator
        },
    );

    let mut sorted_counts: Vec<_> = counts.iter().collect();
    sorted_counts.sort_by(|a: &(&&str, &i32), b: &(&&str, &i32)| b.1.cmp(a.1));
    sorted_counts
        .iter()
        .for_each(|count: &(&&str, &i32)| println!("{:?}", count));

    // get specification and get full list of codes
    // get list of codes not present in these PDFs
}

fn main() {
    let pdfs: Vec<PathBuf> = fs::read_dir("DO_NOT_COMMIT")
        .unwrap()
        .map(|file: Result<fs::DirEntry, std::io::Error>| file.unwrap().path())
        .filter(|path: &PathBuf| path.extension() == Some(OsStr::new("pdf")))
        .collect();

    pdfs.iter().for_each(|pdf: &PathBuf| {
        println!("Parsing data from {:?}", pdf);
        parse_pdf_data(pdf);
    });
}
