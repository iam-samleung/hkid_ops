use crate::{hkid_check_digit::calculate_check_digit, HKID_FULL_REGEX, hkid_prefix::HKIDPrefix};

/// Validates a Hong Kong Identity Card (HKID) number, optionally checking the prefix against known HKID prefixes.
///
/// # Parameters
/// - `hkid_full`: The full HKID string, which may contain parentheses around the check digit (e.g., `"A123456(7)"`, `"AB123456(7)"`).
/// - `must_exist_in_enum`: If `true`, the function will validate that the HKID prefix exists in the `HKIDPrefix` enum. If `false`, any prefix is allowed.
///
/// # Returns
/// - `Ok(true)` if the HKID is valid and the check digit matches.
/// - `Ok(false)` if the check digit does not match (HKID is invalid).
/// - `Err(String)` if the format is incorrect, the check digit is missing, or the prefix is not recognized (when `must_exist_in_enum` is `true`).
///
/// # Errors
/// - Returns `Err` if the format of the HKID is incorrect after removing parentheses (e.g. wrong length or invalid character arrangement).
/// - Returns `Err` if the check digit is missing (which should not occur for valid HKID).
/// - Returns `Err` if the prefix is not recognized and `must_exist_in_enum` is set to `true`.
///
/// # Examples
/// ```ignore
/// // Valid HKID, known prefix, must_exist_in_enum = true
/// assert_eq!(validate_hkid("A123456(7)", true), Ok(true));
///
/// // Invalid check digit
/// assert_eq!(validate_hkid("A123456(8)", true), Ok(false));
///
/// // Unknown prefix, must_exist_in_enum = true
/// assert!(validate_hkid("ZZ123456(7)", true).is_err());
///
/// // Unknown prefix, must_exist_in_enum = false
/// assert_eq!(validate_hkid("ZZ123456(7)", false), Ok(true));
/// ```
///
/// # Details
/// - The function first removes all parentheses from the input, allowing for HKIDs written with or without parentheses.
/// - It then uses a regular expression to check the cleaned string format and extract the prefix, the six digits, and the check digit.
/// - If `must_exist_in_enum` is true, the parsed prefix is checked against the `HKIDPrefix` enum.
/// - The check digit is recalculated from the HKID body and compared to the provided digit. If the check digit is missing, an error is returned.
///
pub fn validate_hkid(hkid_full: &str, must_exist_in_enum: bool) -> Result<bool, String> {
    // Remove all parentheses from the input string (e.g., "A123456(7)" -> "A1234567")
    let cleaned = hkid_full.chars().filter(|&c| c != '(' && c != ')').collect::<String>();

    // Apply regex to the cleaned string and extract capture groups.
    // Regex ensures structure: prefix (1–2 letters), 6 digits, and 1 check digit (A or 0-9).
    let caps = HKID_FULL_REGEX.captures(&cleaned)
        .ok_or_else(|| "Invalid HKID format: incorrect structure.".to_string())?;

    // Destructure captures directly to extract prefix, digits, and provided check digit.
    let (_, [prefix, digits, provided_digit]) = caps.extract();

    // If required, check if the prefix is a known/allowed value.
    if must_exist_in_enum {
        let parsed_prefix = HKIDPrefix::parse(prefix);
        if !parsed_prefix.is_known() {
            return Err(format!("Prefix '{prefix}' is not recognized."));
        }
    }

    // Reassemble the HKID body (prefix + digits) for check digit calculation.
    let hkid_body = format!("{prefix}{digits}");
    // Calculate the expected check digit from the HKID body.
    let calculated_digit = calculate_check_digit(&hkid_body).ok_or_else(|| "Failed to calculate check digit".to_string())?;
    // Extract the provided check digit as a char, or error if missing.
    let provided_digit = provided_digit.chars().next().ok_or_else(|| "Missing check digit".to_string())?;

    Ok(calculated_digit == provided_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hkid_correct() {
        // assuming calculate_check_digit("A123456") returns '3'
        let valid_hkid = "A123456(3)";
        let result = validate_hkid(valid_hkid, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_validate_hkid_incorrect_digit() {
        // Check digit is incorrect
        let invalid_hkid = "A123456(9)";
        let result = validate_hkid(invalid_hkid, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_validate_hkid_invalid_format() {
        // Too short, does not match regex
        let invalid_hkid_format = "A12345"; // less than required chars
        let result = validate_hkid(invalid_hkid_format, false);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid HKID format: incorrect structure."
        );
    }

    #[test]
    fn test_validate_hkid_missing_check_digit() {
        // No check digit at all (e.g., "A123456")
        let missing_digit = "A123456";
        let result = validate_hkid(missing_digit, false);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid HKID format: incorrect structure."
        );
    }

    #[test]
    fn test_validate_hkid_unknown_prefix_with_must_exist() {
        // Prefix 'XX' is not recognized
        let hkid = "XX123456(1)";
        let result = validate_hkid(hkid, true);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Prefix 'XX' is not recognized.");
    }

    #[test]
    fn test_validate_hkid_unknown_prefix_without_must_exist() {
        // Prefix 'ZZ' is not recognized but must_exist_in_enum is false, so should be OK
        let hkid = "ZZ123456(8)";
        let result = validate_hkid(hkid, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_hkid_no_parentheses() {
        // HKID without parentheses, should still be valid
        let valid_hkid = "A1234563";
        let result = validate_hkid(valid_hkid, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
