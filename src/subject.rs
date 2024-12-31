use core::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Subject {
    BiologyTriple,
    ChemistryTriple,
    PhysicsTriple,
    CombinedScience,
}

impl fmt::Display for Subject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let subject_name: String = format!("{:?}", self).replace("Triple", " Triple");
        write!(formatter, "{subject_name}")
    }
}

fn extract_subject_from_filename(filename: &str) -> Subject {
    for word in filename.split(|c: char| !c.is_numeric()) {
        if let Ok(code) = word.parse::<i32>() {
            return get_subject(code);
        }
    }
    panic!("No valid subject code found in filename");
}

pub(crate) fn get_subjects_from_filenames(specification_filenames: &[String]) -> Vec<Subject> {
    specification_filenames
        .iter()
        .map(|filename: &String| extract_subject_from_filename(filename))
        .collect()
}

fn get_subject(code: i32) -> Subject {
    match code {
        8461 => Subject::BiologyTriple,
        8462 => Subject::ChemistryTriple,
        8463 => Subject::PhysicsTriple,
        8464 => Subject::CombinedScience,
        _ => panic!("Invalid subject code"),
    }
}

impl Subject {
    pub(crate) fn get_code(&self) -> i32 {
        match self {
            Subject::BiologyTriple => 8461,
            Subject::ChemistryTriple => 8462,
            Subject::PhysicsTriple => 8463,
            Subject::CombinedScience => 8464,
        }
    }
}
