mod codes;
mod file_reader;
mod specification_pdf;
mod subject;
mod test_pdf;

use codes::{get_code_counts, get_codes};
use file_reader::get_pdf_filenames;
use subject::{extract_subject_from_filename, Subject};
use test_pdf::TestPDF;

fn main() {
    // TODO: do this for each subject individually
    let filenames: Vec<String> = get_pdf_filenames("tests");

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
