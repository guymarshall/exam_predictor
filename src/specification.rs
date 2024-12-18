use core::fmt;

use crate::subject::Subject;

pub(crate) struct Specification {
    filename: String,
    subject: Subject,
    codes: Vec<String>,
}

impl Specification {
    pub(crate) fn new(filename: String, subject: Subject, codes: Vec<String>) -> Specification {
        Specification {
            filename,
            subject,
            codes,
        }
    }
}

impl fmt::Display for Specification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Filename: {}\nSubject: {}\nCodes: {:?}\n",
            self.filename, self.subject, self.codes
        )?;
        Ok(())
    }
}

impl Specification {
    pub(crate) fn get_missing_codes(&self, codes: &[String]) -> Vec<String> {
        self.codes
            .iter()
            .filter(|code: &&String| !codes.contains(code))
            .cloned()
            .collect()
    }
}
