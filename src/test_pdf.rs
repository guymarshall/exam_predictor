use core::fmt;

use crate::subject::Subject;

pub(crate) struct TestPDF {
    pub(crate) filename: String,
    pub(crate) subject: Subject,
    pub(crate) codes: Vec<String>,
    pub(crate) code_counts: Vec<(String, i32)>,
    pub(crate) missing_codes: Vec<String>,
}

impl TestPDF {
    pub(crate) fn new(
        filename: String,
        subject: Subject,
        codes: Vec<String>,
        code_counts: Vec<(String, i32)>,
        missing_codes: Vec<String>,
    ) -> TestPDF {
        TestPDF {
            filename,
            subject,
            codes,
            code_counts,
            missing_codes,
        }
    }
}

impl fmt::Display for TestPDF {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Filename: {}\nSubject: {}\nCodes: {:?}\nMissing Codes: {:?}\n",
            self.filename, self.subject, self.codes, self.missing_codes
        )?;

        for (code, count) in &self.code_counts {
            writeln!(formatter, "  {}: {}", code, count)?;
        }
        Ok(())
    }
}
