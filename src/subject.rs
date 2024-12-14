use core::fmt;

#[derive(Debug)]
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

pub(crate) fn extract_subject_from_filename(filename: &str) -> Subject {
    for word in filename.split(|c: char| !c.is_numeric()) {
        if let Ok(code) = word.parse::<i32>() {
            return get_subject(code);
        }
    }
    panic!("No valid subject code found in filename");
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
