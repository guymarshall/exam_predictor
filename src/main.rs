mod codes;
mod file_reader;
mod specification;
mod subject;
mod test_pdf;

use codes::{get_code_counts, get_codes};
use file_reader::get_filenames;
use specification::Specification;
use subject::{extract_subject_from_filename, Subject};
use test_pdf::TestPDF;

fn main() {
    let specification_filenames: Vec<String> = get_filenames("specifications", "txt");
    let subjects: Vec<Subject> = specification_filenames
        .iter()
        .map(|filename: &String| extract_subject_from_filename(filename))
        .collect();

    for subject in subjects {
        let code: i32 = subject.get_code();
        let filenames: Vec<String> = get_filenames("tests", "pdf")
            .into_iter()
            .filter(|filename: &String| filename.contains(&code.to_string()))
            .collect();

        if filenames.is_empty() {
            continue;
        }

        println!("Subject: {subject}");

        let specification_filename: String = specification_filenames
            .clone()
            .into_iter()
            .filter(|filename: &String| filename.contains(&code.to_string()))
            .last()
            .expect("Failed to get last filename");
        let specification_codes: Vec<String> = get_codes(&specification_filename);
        let specification: Specification =
            Specification::new(specification_filename, subject.clone(), specification_codes);
        let test_pdfs: Vec<TestPDF> = filenames
            .into_iter()
            .map(|filename: String| {
                println!("Parsing data from {:?}", filename);
                let mut codes: Vec<String> = get_codes(&filename);
                codes.sort();
                let current_code_counts: Vec<(String, i32)> = get_code_counts(&codes);
                let mut missing_codes: Vec<String> = specification.get_missing_codes(&codes);
                missing_codes.sort();
                TestPDF::new(
                    filename,
                    subject.clone(),
                    codes,
                    current_code_counts,
                    missing_codes,
                )
            })
            .collect();

        // TODO: remove duplicate codes

        test_pdfs
            .iter()
            .for_each(|test_pdf: &TestPDF| println!("{}", test_pdf));

        // TODO: write list of codes in descending order to file for each subject e.g. biology.txt
    }
}
