use std::collections::HashMap;

use pdf_extract::extract_text;

const DISALLOWED_CHARACTERS: [char; 55] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '–', '/', ' ',
];

const VALID_CHARACTERS: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub(crate) fn get_codes(filename: &String) -> Vec<String> {
    let text: String = extract_text(filename).expect("Failed to open PDF");

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
