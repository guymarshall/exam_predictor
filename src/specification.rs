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
