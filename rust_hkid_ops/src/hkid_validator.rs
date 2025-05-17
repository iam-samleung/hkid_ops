use crate::{hkid_check_digit::calculate_check_digit, hkid_prefix::HKIDPrefix};

/// Validates a Hong Kong Identity Card (HKID) number, optionally checking the prefix against known HKID prefixes.
///
/// # Parameters
/// - `hkid_full`: The full HKID string, which may contain parentheses around the check digit (e.g., `"A123456(7)"`, `"AB123456(7)"`).
/// - `must_exist_in_enum`: If `true`, the function will validate that the HKID prefix exists in the `HKIDPrefix` enum. If `false`, any prefix is allowed.
///
/// # Returns
/// - `Ok(true)` if the HKID is valid and the check digit matches.
/// - `Ok(false)` if the check digit does not match (HKID is invalid).
/// - `Err(String)` if the format is incorrect or the prefix is not recognized (when `must_exist_in_enum` is `true`).
///
/// # Errors
/// - Returns `Err` if the HKID length is not 8 or 9 characters after removing parentheses.
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
/// - The function first removes all parentheses from the input.
/// - It checks that the resulting string is either 8 or 9 characters: prefix (1–2), digits (6), and check digit (1).
/// - It extracts the prefix, the HKID body, and the provided check digit.
/// - If `must_exist_in_enum` is true, it ensures the prefix is recognized by the `HKIDPrefix` enum.
/// - The check digit is calculated from the HKID body and compared with the provided digit.
///
pub fn validate_hkid(hkid_full: &str, must_exist_in_enum: bool) -> Result<bool, String> {
    // Remove parentheses without creating intermediate allocations
    let cleaned: String = hkid_full
        .chars()
        .filter(|&c| c != '(' && c != ')')
        .collect();
    // let cleaned = hkid_full.replace("(", "").replace(")", "");

    // Validate length (8 or 9 characters: prefix(1-2) + digits(6) + check digit(1))
    if !(8..=9).contains(&cleaned.len()) {
        return Err("Invalid HKID format: incorrect length.".to_string());
    }

    // Split the cleaned HKID string into two parts:
    // - hkid_body: all characters except the last one (the main part)
    // - provided_digit: the last character (usually the check digit)
    let (hkid_body, provided_digit) = cleaned.split_at(cleaned.len() - 1);
    // Calculate the length of the HKID prefix (usually 1 or 2 letters).
    // The HKID body (excluding the check digit) always has 7 characters: prefix (1-2 chars) + 6 digits.
    let prefix_len = cleaned.len() - 7;
    // Slice out the prefix from the HKID body using the calculated length.
    let prefix = &hkid_body[..prefix_len];

    // Check prefix validity if required
    if must_exist_in_enum {
        let parsed_prefix = HKIDPrefix::parse(prefix);
        if !parsed_prefix.is_known() {
            return Err(format!("Prefix '{}' is not recognized.", prefix));
        }
    }

    // Calculate and compare check digit
    let calculated_digit = calculate_check_digit(hkid_body);
    Ok(calculated_digit == provided_digit.chars().next().unwrap())
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
        let invalid_hkid = "A123456(9)";
        let result = validate_hkid(invalid_hkid, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_validate_hkid_invalid_format() {
        let invalid_hkid_format = "A12345"; // too short
        let result = validate_hkid(invalid_hkid_format, false);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid HKID format: incorrect length."
        );
    }

    #[test]
    fn test_validate_hkid_unknown_prefix_with_must_exist() {
        let hkid = "XX123456(1)";
        let result = validate_hkid(hkid, true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Prefix 'XX' is not recognized.");
    }
}
