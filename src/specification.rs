use crate::codes::remove_title_from_code;

pub(crate) fn get_missing_codes(
    specification_codes: &[String],
    test_codes: &[String],
) -> Vec<String> {
    specification_codes
        .iter()
        .filter(|code: &&String| !test_codes.contains(&remove_title_from_code(code)))
        .cloned()
        .collect()
}

pub(crate) fn get_subject_specific_specification_filename(
    specification_filenames: &[String],
    subject_code: i32,
) -> String {
    let specification_filename: String = specification_filenames
        .iter()
        .filter(|filename: &&String| filename.contains(&subject_code.to_string()))
        .last()
        .expect("Failed to get last filename")
        .to_string();
    specification_filename
}
