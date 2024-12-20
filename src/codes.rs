use std::collections::HashMap;

use pdf_extract::extract_text;
use crate::file_reader::extract_text_from_txt;

const VALID_CHARACTERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub(crate) fn get_codes(filename: &String) -> Vec<String> {
    let extension: &str = filename.split(".").last().unwrap();

    let text: String = match extension {
        "pdf" => extract_text(filename).expect("Failed to open PDF"),
        "txt" => extract_text_from_txt(filename),
        _ => panic!("Invalid extension"),
    };

    let codes: Vec<String> = text
        .lines()
        .map(|line: &str| line.trim().to_string())
        .filter(|line: &String| line.contains("."))
        .filter(|line: &String| line.contains(VALID_CHARACTERS))
        .map(|line: String| line.split_whitespace().next().unwrap().to_string()) // remove leading titles from specification codes
        .filter(|line: &String| line.len() == 5 || line.len() == 7)
        .filter(|line: &String| line.matches('.').count() == 2 || line.matches('.').count() == 3)
        .collect();

    codes
}

pub(crate) fn get_code_counts(codes: &[String]) -> Vec<(String, i32)> {
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
