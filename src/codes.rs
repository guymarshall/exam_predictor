use std::collections::HashMap;

use crate::file_reader::extract_text_from_txt;
use pdf_extract::extract_text;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Extension {
    Pdf,
    Txt,
}

fn get_text(extension: &Extension, filename: &String) -> String {
    match extension {
        Extension::Pdf => extract_text(filename).expect("Failed to open PDF"),
        Extension::Txt => extract_text_from_txt(filename),
    }
}

pub(crate) fn remove_title_from_code(line: &str) -> String {
    line.split_whitespace().next().unwrap().to_string()
}

fn get_filtered_codes(text: String, extension: &Extension) -> Vec<String> {
    let regex: Regex = Regex::new(r"^4\.\d+\.\d+\.\d+").unwrap();

    let codes_with_titles: Vec<String> = text
        .lines()
        .map(|line: &str| line.trim())
        .filter(|line: &&str| regex.is_match(line))
        .map(|line: &str| line.to_string())
        .collect();

    match extension {
        Extension::Txt => codes_with_titles,
        Extension::Pdf => codes_with_titles
            .iter()
            .map(|line: &String| remove_title_from_code(line))
            .collect(),
    }
}

pub(crate) fn get_codes(filename: &String) -> Vec<String> {
    let extension: Extension = {
        let extension_str: &str = filename.split(".").last().unwrap();

        match extension_str {
            "pdf" => Extension::Pdf,
            "txt" => Extension::Txt,
            _ => panic!("Invalid extension"),
        }
    };

    let text: String = get_text(&extension, filename);

    get_filtered_codes(text, &extension)
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
