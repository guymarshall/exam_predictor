mod codes;
mod file_reader;
mod file_writer;
mod specification;
mod subject;
mod test_pdf;

use codes::{get_code_counts, get_codes};
use file_reader::{get_filenames, get_subject_specific_filenames};
use file_writer::write_to_file;
use rayon::prelude::*;
use specification::{get_missing_codes, get_subject_specific_specification_filename};
use subject::{get_subjects_from_filenames, Subject};
use test_pdf::TestPDF;

fn main() {
    let specification_filenames: Vec<String> = get_filenames("specifications", "txt");
    let subjects: Vec<Subject> = get_subjects_from_filenames(&specification_filenames);

    for subject in subjects {
        let subject_code: i32 = subject.get_code();
        let subject_filenames: Vec<String> =
            get_subject_specific_filenames("tests", "pdf", subject_code);

        if subject_filenames.is_empty() {
            continue;
        }

        println!("Subject: {subject}");

        let specification_filename: String =
            get_subject_specific_specification_filename(&specification_filenames, subject_code);
        let mut specification_codes: Vec<String> = get_codes(&specification_filename);
        specification_codes.sort();
        specification_codes.dedup();

        let test_pdfs: Vec<TestPDF> = subject_filenames
            .into_par_iter()
            .map(|subject_filename: String| {
                println!("Parsing data from {:?}", subject_filename);
                let mut test_codes: Vec<String> = get_codes(&subject_filename);
                test_codes.sort();

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
            .for_each(|test_pdf: &TestPDF| write_to_file(test_pdf));
    }
}
