mod codes;
mod file_reader;
mod specification;
mod subject;
mod test_pdf;

use codes::{get_code_counts, get_codes};
use file_reader::get_filenames;
use specification::get_missing_codes;
use subject::{extract_subject_from_filename, Subject};
use test_pdf::TestPDF;

fn main() {
    let specification_filenames: Vec<String> = get_filenames("specifications", "txt");
    let subjects: Vec<Subject> = specification_filenames
        .iter()
        .map(|filename: &String| extract_subject_from_filename(filename))
        .collect();

    for subject in subjects {
        let subject_code: i32 = subject.get_code();
        let subject_filenames: Vec<String> = get_filenames("tests", "pdf")
            .into_iter()
            .filter(|filename: &String| filename.contains(&subject_code.to_string()))
            .collect();

        if subject_filenames.is_empty() {
            continue;
        }

        println!("Subject: {subject}");

        let specification_filename: String = specification_filenames
            .clone()
            .into_iter()
            .filter(|filename: &String| filename.contains(&subject_code.to_string()))
            .last()
            .expect("Failed to get last filename");
        let mut specification_codes: Vec<String> = get_codes(&specification_filename);
        specification_codes.sort();
        specification_codes.dedup();

        let test_pdfs: Vec<TestPDF> = subject_filenames
            .into_iter()
            .map(|subject_filename: String| {
                println!("Parsing data from {:?}", subject_filename);
                let mut test_codes: Vec<String> = get_codes(&subject_filename);
                test_codes.sort();
                test_codes.dedup();

                let code_counts: Vec<(String, i32)> = get_code_counts(&test_codes);
                let mut missing_codes: Vec<String> =
                    get_missing_codes(&specification_codes, &test_codes);
                missing_codes.sort();
                missing_codes.dedup();

                TestPDF::new(
                    subject_filename,
                    subject.clone(),
                    test_codes,
                    code_counts,
                    missing_codes,
                )
            })
            .collect();

        test_pdfs
            .iter()
            .for_each(|test_pdf: &TestPDF| println!("{}", test_pdf));

        // TODO: write list of codes in descending order to file for each subject e.g. biology.txt
    }
}
