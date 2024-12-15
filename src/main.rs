mod codes;
mod file_reader;
mod specification_pdf;
mod subject;
mod test_pdf;

use codes::{get_code_counts, get_codes};
use file_reader::get_pdf_filenames;
use specification_pdf::SpecificationPDF;
use subject::{extract_subject_from_filename, Subject};
use test_pdf::TestPDF;

fn main() {
    let specification_filenames: Vec<String> = get_pdf_filenames("specifications");
    let subjects: Vec<Subject> = specification_filenames
        .iter()
        .map(|filename: &String| extract_subject_from_filename(filename))
        .collect();

    for subject in subjects {
        let code: i32 = subject.get_code();
        let filenames: Vec<String> = get_pdf_filenames("tests")
            .into_iter()
            .filter(|filename: &String| filename.contains(&code.to_string()))
            .collect();

        if !filenames.is_empty() {
            println!("Subject: {subject}");
        }

        let specification_filename: String = specification_filenames
            .clone()
            .into_iter()
            .filter(|filename: &String| filename.contains(&code.to_string()))
            .last()
            .expect("Failed to get last filename");
        ////! ERROR in crate: PdfError(Decryption(UnsupportedEncryption)) for specifications due to PDF-1.7 format being used. PDF-1.6 format works fine.
        // let specification_codes: Vec<String> = get_codes(&specification_filename);
        // let specification_pdf: SpecificationPDF =
        //     SpecificationPDF::new(specification_filename, subject.clone(), specification_codes);
        // TODO: for each subject -> get list of codes that are missing from the PDFs
        let test_pdfs: Vec<TestPDF> = filenames
            .into_iter()
            .map(|filename: String| {
                println!("Parsing data from {:?}", filename);
                let codes: Vec<String> = get_codes(&filename);
                let current_code_counts: Vec<(String, i32)> = get_code_counts(&codes);
                TestPDF::new(filename, subject.clone(), codes, current_code_counts)
            })
            .collect();

        test_pdfs
            .iter()
            .for_each(|test_pdf: &TestPDF| println!("{}", test_pdf));

        // TODO: write list of codes in descending order to file for each subject e.g. biology.txt
    }
}
