use rand::rng;
use rand::seq::IndexedRandom;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::{hkid_check_digit::calculate_check_digit, hkid_prefix::HKIDPrefix};

/// Generates a random Hong Kong Identity Card (HKID) number with the specified or random prefix.
///
/// This function creates a validly-formatted HKID string by:
/// 1. Validating the prefix (optionally enforcing that it is a known HKID prefix).
/// 2. Appending six random digits to the prefix.
/// 3. Calculating the correct check digit for the generated body.
/// 4. Returning the final HKID in the standard format (with the check digit in parentheses).
///
/// # Arguments
/// - `prefix`: The prefix to use for the HKID (e.g., `"A"`, `"AB"`, `"WX"`). If `None`, a random prefix is generated.
/// - `must_exist_in_enum`: If `true`, the function returns an error if the prefix is not recognized by `HKIDPrefix`.
///
/// # Random Prefix Logic
/// - If `prefix` is `None` and `must_exist_in_enum` is `true`, a random known prefix is selected from `HKIDPrefix`.
/// - If `prefix` is `None` and `must_exist_in_enum` is `false`, a random one-letter or two-letter uppercase prefix is generated (e.g., `"A"`, `"ZQ"`).
///
/// # Returns
/// - `Ok(String)`: A randomly generated HKID string in the format `PREFIXdddddd(C)`, where `d` is a digit and `C` is the check digit.
/// - `Err(String)`: If the prefix is not recognized and `must_exist_in_enum` is `true`.
///
/// # Errors
/// Returns an error if `must_exist_in_enum` is set and the prefix is not recognized as a valid `HKIDPrefix`.
///
/// # Example
/// ```ignore
/// use hkid_ops::utils::{generate_hkid, HKIDPrefix};
///
/// // Generate with known prefix
/// let hkid = generate_hkid(Some("A"), true).unwrap();
/// assert!(hkid.starts_with("A"));
///
/// // Generate with random known prefix
/// let hkid_random_known = generate_hkid(None, true).unwrap();
/// // (prefix is guaranteed to be in the HKIDPrefix enum)
///
/// // Generate with random one- or two-letter prefix
/// let hkid_any = generate_hkid(None, false).unwrap();
/// assert!(hkid_any.len() >= 10 && hkid_any.len() <= 11);
///
/// // Custom or unknown prefix (allowed when must_exist_in_enum is false)
/// let custom = generate_hkid(Some("ZZ"), false).unwrap();
/// assert!(custom.starts_with("ZZ"));
///
/// // Custom prefix not allowed
/// assert!(generate_hkid(Some("ZZ"), true).is_err());
/// ```
///
/// # Note
/// - The random number generator used must provide `random_range`.
/// - The check digit calculation uses your implementation of `calculate_check_digit`.
///
pub fn generate_hkid(
    prefix: Option<&str>,
    must_exist_in_enum: bool,
) -> Result<String, String> {
    let mut rng = rng();

    // Determine prefix string
    let prefix_str = match prefix {
        Some(px) => {
            let parsed_prefix = HKIDPrefix::parse(px);
            if must_exist_in_enum && !parsed_prefix.is_known() {
                return Err(format!("Prefix '{}' is not recognized", px));
            }
            parsed_prefix.as_str()
        }
        None => {
            if must_exist_in_enum {
                let valid_prefixes: Vec<String> = HKIDPrefix::iter()
                    .filter(|variant| variant.is_known())
                    .map(|variant| variant.as_str())
                    .collect();
                valid_prefixes
                    .choose(&mut rng)
                    .expect("No valid prefixes in HKIDPrefix enum")
                    .clone()
            } else {
                // Generate a random one- or two-letter uppercase prefix
                if rng.random_range(0..2) == 0 {
                    // One-letter prefix
                    let letter = rng.random_range(b'A'..=b'Z') as char;
                    letter.to_string()
                } else {
                    // Two-letter prefix
                    let letter1 = rng.random_range(b'A'..=b'Z') as char;
                    let letter2 = rng.random_range(b'A'..=b'Z') as char;
                    format!("{}{}", letter1, letter2)
                }
            }
        }
    };

    // Compose HKID body
    let mut hkid_body = prefix_str;

    // Append 6 random digits
    for _ in 0..6 {
        hkid_body.push_str(&rng.random_range(0..10).to_string());
    }

    let check_digit = calculate_check_digit(&hkid_body);

    Ok(format!("{}({})", hkid_body, check_digit))
}

#[cfg(test)]
mod tests {
    use super::generate_hkid;

    // Helper to check HKID format: PREFIX + 6 digits + (check digit)
    fn is_valid_format(hkid: &str) -> bool {
        let parts: Vec<_> = hkid.split(['(', ')']).collect();

        if parts.len() != 3 {
            return false;
        }

        let prefix_digits = parts[0];
        let check_digit = parts[1];

        if check_digit.len() != 1 {
            return false;
        }

        let prefix_len = prefix_digits.len();

        if prefix_len < 7 || prefix_len > 8 {
            return false;
        }
        // Last 6 chars before ( should be digits
        prefix_digits
            .chars()
            .rev()
            .take(6)
            .all(|c| c.is_ascii_digit())
    }

    #[test]
    fn test_generate_hkid_with_known_prefix() {
        let prefix = "A";
        let result = generate_hkid(Some(prefix), true);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        assert!(hkid.starts_with(prefix));
        assert!(is_valid_format(&hkid));
    }

    #[test]
    fn test_generate_hkid_with_two_letter_known_prefix() {
        let prefix = "WX";
        let result = generate_hkid(Some(prefix), true);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        assert!(hkid.starts_with(prefix));
        assert!(is_valid_format(&hkid));
    }

    #[test]
    fn test_generate_hkid_with_custom_prefix_allowed() {
        let prefix = "ZZ";
        let result = generate_hkid(Some(prefix), false);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        assert!(hkid.starts_with(prefix));
        assert!(is_valid_format(&hkid));
    }

    #[test]
    fn test_generate_hkid_with_random_known_prefix() {
        // Should generate HKID with a valid known prefix
        let result = generate_hkid(None, true);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        // At least one digit after prefix
        assert!(is_valid_format(&hkid));
    }

    #[test]
    fn test_generate_hkid_with_random_any_prefix() {
        // Should generate HKID with a 1- or 2-letter uppercase prefix
        let result = generate_hkid(None, false);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        assert!(is_valid_format(&hkid));

        let prefix_len = hkid.find(|c: char| c.is_ascii_digit()).unwrap();

        assert!(prefix_len == 1 || prefix_len == 2);

        let prefix = &hkid[..prefix_len];
        for c in prefix.chars() {
            assert!(c.is_ascii_uppercase());
        }
    }

    #[test]
    fn test_generate_hkid_with_empty_prefix() {
        // Should treat empty string as unknown and return error when must_exist_in_enum is true
        let result = generate_hkid(Some(""), true);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_hkid_with_lowercase_prefix() {
        // Should treat lowercase unknown prefix as not recognized
        let result = generate_hkid(Some("a"), true);
        assert!(result.is_err());
    }
}
