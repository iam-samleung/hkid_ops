use rand::{rng, Rng, seq::IndexedRandom};
use strum::IntoEnumIterator;

use crate::{hkid_check_digit::calculate_check_digit, hkid_prefix::HKIDPrefix};

/// Generates a random uppercase ASCII letter ('A' to 'Z').
///
/// # Arguments
///
/// * `rng` - A mutable reference to a random number generator that implements the `Rng` trait.
///
/// # Returns
///
/// A randomly selected uppercase letter as a `char`.
///
/// # Example
///
/// ```ignore
/// use rand::thread_rng;
/// use hkid_ops::random_uppercase_letter;
///
/// let mut rng = thread_rng();
/// let letter = random_uppercase_letter(&mut rng);
/// assert!(letter.is_ascii_uppercase());
/// ```
fn random_uppercase_letter<R: Rng + ?Sized>(rng: &mut R) -> char {
    rng.random_range(b'A'..=b'Z') as char
}

/// Selects a random known HKID prefix from the `HKIDPrefix` enum.
///
/// This function filters all variants of `HKIDPrefix` to include only those that are recognized
/// as "known" (using `is_known`), and returns one at random as an owned `String`.
/// Returns `None` if there are no known prefixes.
///
/// # Arguments
/// * `rng` - A mutable reference to a random number generator implementing the `Rng` trait.
///
/// # Returns
/// * `Some(String)` - A randomly selected known prefix as an owned `String`.
/// * `None` - If no known prefixes are available.
///
/// # Example
/// ```ignore
/// use rand::thread_rng;
///
/// let mut rng = thread_rng();
/// if let Some(prefix) = random_known_prefix(&mut rng) {
///     println!("Random known prefix: {}", prefix);
/// }
/// ```
fn random_known_prefix<R: Rng>(rng: &mut R) -> Option<String> {
    let valid_prefixes = HKIDPrefix::iter()
        .filter(HKIDPrefix::is_known)
        .map(|variant| variant.as_str())
        .collect::<Vec<String>>();

    valid_prefixes.choose(rng).cloned()
}

/// Generates a random one-letter or two-letter uppercase prefix for HKID.
///
/// Randomly chooses either one or two uppercase ASCII letters ('A' to 'Z') to form a prefix string.
/// The length is chosen at random (50% chance for each).
///
/// # Arguments
/// * `rng` - A mutable reference to a random number generator implementing the `Rng` trait.
///
/// # Returns
/// * `String` - A randomly generated prefix consisting of one or two uppercase letters.
///
/// # Example
/// ```ignore
/// let mut rng = thread_rng();
/// let prefix = random_prefix(&mut rng);
/// assert!(prefix.len() == 1 || prefix.len() == 2);
/// assert!(prefix.chars().all(|c| c.is_ascii_uppercase()));
/// ```
fn random_prefix<R: Rng>(rng: &mut R) -> String {
    let len = if rng.random_bool(0.5) { 1 } else { 2 };

    (0..len).map(|_| random_uppercase_letter(rng))
        .collect()
}

/// Generates a random Hong Kong Identity Card (HKID) number, using a specified or random prefix.
///
/// This function creates a valid HKID string by:
/// 1. Validating the prefix (enforcing that it is a known HKID prefix if required).
/// 2. Appending six random digits to the prefix.
/// 3. Calculating the correct check digit for the generated HKID body.
/// 4. Returning the final HKID in the format `<PREFIX>dddddd(C)`, where `d` is a digit and `C` is the check digit.
///
/// # Arguments
/// - `prefix`: An optional prefix for the HKID (e.g., `"A"`, `"AB"`). If `None`, a random prefix is generated.
/// - `must_exist_in_enum`: If `true`, the function returns an error if the prefix is not recognized by `HKIDPrefix`.
///
/// # Prefix Logic
/// - If `prefix` is `Some` and `must_exist_in_enum` is `true`, the prefix is validated against known HKID prefixes.
/// - If `prefix` is `Some` and `must_exist_in_enum` is `false`, the prefix is used as is.
/// - If `prefix` is `None` and `must_exist_in_enum` is `true`, a random known prefix is selected from `HKIDPrefix`.
/// - If `prefix` is `None` and `must_exist_in_enum` is `false`, a random one- or two-letter uppercase prefix is generated.
///
/// # Returns
/// - `Ok(String)`: A randomly generated HKID string in the format `<PREFI>Xdddddd(C)`.
/// - `Err(String)`: If the prefix is not recognized and `must_exist_in_enum` is `true`.
///
/// # Errors
/// Returns an error if `must_exist_in_enum` is set and the prefix is not recognized as a valid `HKIDPrefix`.
///
/// # Example
/// ```ignore
/// use hkid_ops::utils::{generate_hkid, HKIDPrefix};
///
/// // Generate with a known prefix
/// let hkid = generate_hkid(Some("A"), true).unwrap();
/// assert!(hkid.starts_with("A"));
///
/// // Generate with a random known prefix
/// let hkid_random_known = generate_hkid(None, true).unwrap();
/// // (Prefix is guaranteed to be in the HKIDPrefix enum)
///
/// // Generate with a random one- or two-letter prefix
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
pub fn generate_hkid(prefix: Option<&str>, must_exist_in_enum: bool) -> Result<String, String> {
    let mut rng = rng();

    // Determine the HKID prefix string based on user input and requirements:
    // - If a prefix is provided and must exist in the enum, validate it and return an error if unrecognized.
    // - If a prefix is provided and enum validation is not required, use it directly.
    // - If no prefix is provided but must exist in the enum, randomly select a valid known prefix.
    // - If no prefix is provided and any prefix is allowed, generate a random one- or two-letter uppercase prefix.
    let prefix_str = match (prefix, must_exist_in_enum) {
        (Some(px), true) => {
            let parsed_prefix = HKIDPrefix::parse(px);

            if !parsed_prefix.is_known() {
                return Err(format!("Prefix '{px}' is not recognized"));
            }

            parsed_prefix.as_str()
        }
        (Some(px), false) => {
            HKIDPrefix::parse(px).as_str()
        }
        (None, true) => {
            random_known_prefix(&mut rng)
                .ok_or_else(|| "No valid prefixes in HKIDPrefix enum".to_string())?
                .to_string()
        }
        (None, false) => random_prefix(&mut rng),
    };

    // Generate 6 random digits
    let digits = (0..6).map(|_| rng.random_range(0..10).to_string()).collect::<String>();
    let hkid_body = format!("{prefix_str}{digits}");
    let check_digit = calculate_check_digit(&hkid_body).ok_or_else(|| "Failed to calculate check digit")?;

    Ok(format!("{hkid_body}({check_digit})"))
}

#[cfg(test)]
mod tests {
    use rand::rng;
    use super::{generate_hkid, random_uppercase_letter, random_prefix, random_known_prefix, HKIDPrefix};

    #[test]
    fn test_random_uppercase_letter_range() {
        let mut rng = rng();
        // Run the function multiple times to check its range and type
        for _ in 0..100 {
            let c = random_uppercase_letter(&mut rng);

            assert!(
                c.is_ascii_uppercase(),
                "Generated char '{}' is not ASCII uppercase", c
            );
            assert!(
                ('A'..='Z').contains(&c),
                "Generated char '{}' is not in 'A'..='Z'", c
            );
        }
    }

    #[test]
    fn test_random_prefix_length_and_case() {
        let mut rng = rng();
        for _ in 0..100 {
            let prefix = random_prefix(&mut rng);
            assert!(
                prefix.len() == 1 || prefix.len() == 2,
                "Prefix length should be 1 or 2, got '{}'", prefix
            );
            assert!(
                prefix.chars().all(|c| c.is_ascii_uppercase()),
                "Prefix '{}' should be all uppercase", prefix
            );
        }
    }

    #[test]
    fn test_random_known_prefix_is_known() {
        let mut rng = rng();
        for _ in 0..20 {
            if let Some(prefix) = random_known_prefix(&mut rng) {
                // Must be one of the known prefixes in the enum
                let parsed = HKIDPrefix::parse(&prefix);
                assert!(parsed.is_known(), "random_known_prefix produced unknown prefix: {}", prefix);
            } else {
                // If no known prefixes exist, that's fine (empty enum)
            }
        }
    }

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

        assert!(is_valid_format(&hkid));

        // The prefix should be known
        let prefix_len = hkid.find(|c: char| c.is_ascii_digit()).unwrap();
        let prefix = &hkid[..prefix_len];

        assert!(HKIDPrefix::parse(prefix).is_known());
    }

    #[test]
    fn test_generate_hkid_with_random_any_prefix() {
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

    #[test]
    fn test_generate_hkid_with_unknown_prefix_not_allowed() {
        // Custom prefix "ZZ" not allowed if must_exist_in_enum is true
        let result = generate_hkid(Some("ZZ"), true);
        assert!(result.is_err());
    }
}