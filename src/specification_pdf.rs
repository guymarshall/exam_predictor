use core::fmt;

use crate::subject::Subject;

pub(crate) struct SpecificationPDF {
    filename: String,
    subject: Subject,
    codes: Vec<String>,
}

impl SpecificationPDF {
    pub(crate) fn new(filename: String, subject: Subject, codes: Vec<String>) -> SpecificationPDF {
        SpecificationPDF {
            filename,
            subject,
            codes,
        }
    }
}

impl fmt::Display for SpecificationPDF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Filename: {}\nSubject: {}\nCodes: {:?}\n",
            self.filename, self.subject, self.codes
        )?;
        Ok(())
    }
}
