use crate::test_pdf::TestPDF;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn strip_directory_and_extension(filename: &str) -> &str {
    filename
        .strip_suffix(".pdf")
        .unwrap()
        .strip_prefix("tests/")
        .unwrap()
}

pub(crate) fn write_to_file(test_pdf: &TestPDF) {
    const OUTPUT_DIRECTORY: &str = "output/";

    if let Err(e) = fs::create_dir_all(OUTPUT_DIRECTORY) {
        panic!("Failed to create output directory: {}", e);
    }

    let filepath: PathBuf = Path::new(OUTPUT_DIRECTORY).join(format!(
        "{}.txt",
        strip_directory_and_extension(&test_pdf.filename)
    ));

    let mut file: File = match File::create(&filepath) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to create file {}: {}", filepath.display(), e);
        }
    };

    if writeln!(
        file,
        "{}: {}",
        strip_directory_and_extension(&test_pdf.filename),
        test_pdf.subject
    )
    .is_err()
        || writeln!(file, "\nMost Frequent Codes:").is_err()
    {
        panic!("Failed to write to file {}", filepath.display());
    }

    for (code, count) in &test_pdf.code_counts {
        if writeln!(file, "  {}: {}", code, count).is_err() {
            panic!("Failed to write code_counts to file {}", filepath.display());
        }
    }

    if writeln!(file, "\nMissing Codes:").is_err() {
        panic!("Failed to write to file {}", filepath.display());
    }

    for code in &test_pdf.missing_codes {
        if writeln!(file, "  {}", code).is_err() {
            panic!(
                "Failed to write missing_codes to file {}",
                filepath.display()
            );
        }
    }
}
