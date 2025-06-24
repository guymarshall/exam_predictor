use std::collections::HashMap;

use pdf_extract::extract_text;
use regex::Regex;

fn get_text(filename: &String) -> String {
    extract_text(filename).expect("Failed to open PDF")
}

pub(crate) fn remove_title_from_code(line: &str) -> String {
    line.split_whitespace().next().unwrap().to_string()
}

fn get_filtered_codes(text: String) -> Vec<String> {
    let regex: Regex = Regex::new(r"^4\.\d+\.\d+\.\d+").unwrap();

    let codes_with_titles: Vec<String> = text
        .lines()
        .map(|line: &str| line.trim())
        .filter(|line: &&str| regex.is_match(line))
        .map(|line: &str| line.to_string())
        .collect();

    codes_with_titles
        .iter()
        .map(|line: &String| remove_title_from_code(line))
        .collect()
}

pub(crate) fn get_codes(filename: &String) -> Vec<String> {
    let text: String = get_text(filename);

    get_filtered_codes(text)
}

pub(crate) fn get_code_counts(codes: &[String]) -> Vec<(String, i32)> {
    let counts: HashMap<String, i32> = codes.iter().fold(
        HashMap::new(),
        |mut accumulator: HashMap<String, i32>, pattern: &String| {
            *accumulator
                .entry(remove_title_from_code(pattern).to_string())
                .or_insert(0) += 1;
            accumulator
        },
    );

    let mut sorted_counts: Vec<_> = counts.into_iter().collect();
    sorted_counts.sort_by(|a: &(String, i32), b: &(String, i32)| b.1.cmp(&a.1));
    sorted_counts
}
