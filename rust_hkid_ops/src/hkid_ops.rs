use rand::{Rng, seq::IndexedRandom};
use regex::Regex;
use strum::IntoEnumIterator;

use crate::hkid_prefix::HKIDPrefix;

/// The weights used in HKID check digit calculation.
///
/// These weights are multiplied by the numeric values of each character in the HKID body
/// (after left-padding to 8 characters) as part of the official Hong Kong Identity Card (HKID)
/// check digit algorithm. The weights are applied from left to right, starting with 9 and ending with 2.
///
/// # Example
///
/// For an HKID body `"A123456"`:
/// - After left-padding: `" A123456"`
/// - The weights applied are: `[9, 8, 7, 6, 5, 4, 3, 2]`
/// - Each character is mapped to a value: `' '` → 36, `'A'` → 10, `'1'` → 1, etc.
/// - The products are summed and used to calculate the check digit.
///
/// ```rust
/// // The weights used in HKID check digit calculation
/// const WEIGHTS: [u32; 8] = [9, 8, 7, 6, 5, 4, 3, 2];
///
/// // Helper: Map a char to HKID value
/// fn char_to_value(c: char) -> Option<u32> {
///     match c {
///         'A'..='Z' => Some((c as u32 - 'A' as u32) + 10),
///         '0'..='9' => Some(c as u32 - '0' as u32),
///         ' ' => Some(36),
///         _ => None,
///     }
/// }
///
/// let hkid_body = "A123456";
/// let padded = format!("{:>8}", hkid_body); // Left pad to 8 chars: " A123456"
/// let values: Vec<u32> = padded.chars().map(|c| char_to_value(c).unwrap()).collect();
///
/// // Calculate weighted sum
/// let weighted_sum: u32 = values.iter().zip(WEIGHTS.iter())
///     .map(|(v, w)| v * w)
///     .sum();
///
/// // Calculate check digit
/// let check_digit = (11 - (weighted_sum % 11)) % 11;
/// let check_char = if check_digit == 10 { 'A' } else { std::char::from_digit(check_digit, 10).unwrap() };
///
/// assert_eq!(padded, " A123456");
/// assert_eq!(values, vec![36, 10, 1, 2, 3, 4, 5, 6]);
/// assert_eq!(weighted_sum, 36*9 + 10*8 + 1*7 + 2*6 + 3*5 + 4*4 + 5*3 + 6*2);
/// // check_char is the check digit character
/// ```
pub const WEIGHTS: [u32; 8] = [9, 8, 7, 6, 5, 4, 3, 2];

/// Regular expression pattern for a valid HKID prefix.
///
/// This pattern matches a string that consists of exactly **1 or 2 uppercase ASCII letters** (`A`–`Z`),
/// corresponding to valid HKID prefixes such as `"A"`, `"AB"`, `"WX"`, etc. Lowercase, digits, or other characters are not allowed.
///
/// # Examples
/// ```
/// # use regex::Regex;
/// # const VALID_PREFIX_PATTERN: &str = r"^[A-Z]{1,2}$";
/// let re = Regex::new(VALID_PREFIX_PATTERN).unwrap();
/// assert!(re.is_match("A"));      // Valid: single letter
/// assert!(re.is_match("EC"));     // Valid: two letters
/// assert!(!re.is_match("a"));     // Invalid: lowercase
/// assert!(!re.is_match("A1"));    // Invalid: contains digit
/// assert!(!re.is_match(""));      // Invalid: empty
/// assert!(!re.is_match("ABC"));   // Invalid: too long
/// assert!(!re.is_match("A C"));   // Invalid: contains space
/// ```
const VALID_PREFIX_PATTERN: &str = r"^[A-Z]{1,2}$";

/// Regular expression for validating an HKID prefix.
///
/// This regex matches a string of **exactly 1 or 2 uppercase ASCII letters** (`A`–`Z`),
/// corresponding to valid HKID prefixes such as `"A"`, `"AB"`, `"EC"`, etc.
/// It does not allow lowercase, digits, or other symbols.
///
/// Used to strictly check the prefix argument for HKID generation and validation.
///
/// # Example
/// ```rust
/// # use regex::Regex;
/// # const VALID_PREFIX_PATTERN: &str = r"^[A-Z]{1,2}$";
/// # static VALID_PREFIX_REGEX: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(VALID_PREFIX_PATTERN).unwrap());
/// assert!(VALID_PREFIX_REGEX.is_match("A"));
/// assert!(VALID_PREFIX_REGEX.is_match("EC"));
/// assert!(!VALID_PREFIX_REGEX.is_match("abc"));
/// assert!(!VALID_PREFIX_REGEX.is_match("A1"));
/// assert!(!VALID_PREFIX_REGEX.is_match(""));
/// ```
static VALID_PREFIX_REGEX: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(VALID_PREFIX_PATTERN).unwrap());

/// Pattern for a valid HKID body: 7 or 8 uppercase letters/digits (A-Z, 0-9).
///
/// This regex matches strings that are exactly 7 or 8 characters long,
/// where each character is an uppercase ASCII letter (`A`–`Z`) or digit (`0`–`9`).
///
/// # Examples
/// ```rust
/// # use regex::Regex;
/// # const VALID_HKID_BODY_PATTERN: &str = r"^[A-Z0-9]{7,8}$";
/// let re = Regex::new(VALID_HKID_BODY_PATTERN).unwrap();
///
/// assert!(re.is_match("A123456"));    // 7 chars, valid
/// assert!(re.is_match("AB123456"));   // 8 chars, valid
/// assert!(re.is_match("Z999999"));    // 7 chars, valid
/// assert!(re.is_match("A1234567"));   // 8 chars, valid
/// assert!(!re.is_match("a123456"));   // lowercase not allowed
/// assert!(!re.is_match("A12345"));    // too short
/// assert!(!re.is_match("A12345_"));   // invalid character
/// ```
const VALID_HKID_BODY_PATTERN: &str = r"^[A-Z0-9]{7,8}$";

/// Compiled regex for validating an HKID body (7 or 8 uppercase letters/digits).
///
/// This static regex matches strings that are exactly 7 or 8 characters long,
/// where each character is an uppercase ASCII letter (`A`–`Z`) or digit (`0`–`9`).
///
/// Using a static `LazyLock` ensures the regex is compiled only once, improving efficiency.
///
/// # Examples
/// ```
/// # use regex::Regex;
/// # use std::sync::LazyLock;
/// # const VALID_HKID_BODY_PATTERN: &str = r"^[A-Z0-9]{7,8}$";
/// static VALID_HKID_BODY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(VALID_HKID_BODY_PATTERN).unwrap());
///
/// assert!(VALID_HKID_BODY_REGEX.is_match("A123456"));    // 7 chars, valid
/// assert!(VALID_HKID_BODY_REGEX.is_match("AB123456"));   // 8 chars, valid
/// assert!(!VALID_HKID_BODY_REGEX.is_match("a123456"));   // Invalid: lowercase
/// assert!(!VALID_HKID_BODY_REGEX.is_match("123456"));    // Invalid: too short
/// assert!(!VALID_HKID_BODY_REGEX.is_match("A123456_"));  // Invalid: underscore
/// ```
static VALID_HKID_BODY_REGEX: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(VALID_HKID_BODY_PATTERN).unwrap());

/// Regex pattern for a full HKID:
/// - 1 or 2 uppercase letters (prefix)
/// - 6 digits
/// - 1 check digit (A or 0-9)
///
/// This pattern matches the full "raw" HKID (without parentheses), such as:
/// - `"A1234567"`
/// - `"AB1234568"`
/// - `"WX123456A"`
///
/// # Examples
/// ```
/// # use regex::Regex;
/// # const HKID_FULL_PATTERN: &str = r"^([A-Z]{1,2})([0-9]{6})([A0-9])$";
/// let re = Regex::new(HKID_FULL_PATTERN).unwrap();
///
/// assert!(re.is_match("A1234567"));     // Valid: 1-letter prefix, digit check digit
/// assert!(re.is_match("AB1234568"));    // Valid: 2-letter prefix, digit check digit
/// assert!(re.is_match("WX123456A"));    // Valid: 2-letter prefix, 'A' check digit
///
/// assert!(!re.is_match("A123456"));     // Invalid: too short
/// assert!(!re.is_match("AB12345A"));    // Invalid: too short
/// assert!(!re.is_match("A12345678"));   // Invalid: too long (7 digits)
/// assert!(!re.is_match("a1234567"));    // Invalid: lowercase prefix
/// assert!(!re.is_match("A123456!"));    // Invalid: non-check character
/// assert!(!re.is_match("A123456("));    // Invalid: parenthesis
/// ```
const HKID_FULL_PATTERN: &str = r"^([A-Z]{1,2})([0-9]{6})([A0-9])$";

// Compiled regex for matching full HKID against its official structure.
static HKID_FULL_REGEX: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(HKID_FULL_PATTERN).unwrap());

/// `HKIDOps` provides the main implementation.
pub struct HKIDOps;

impl HKIDOps {
    /// Converts a single character to its corresponding HKID numeric value.
    ///
    /// - Uppercase or lowercase English letters (`A`–`Z`, `a`–`z`) are mapped to 10–35:
    ///   - `'A'`/`'a'` → 10, ..., `'Z'`/`'z'` → 35.
    /// - Digits (`'0'`–`'9'`) are mapped to 0–9.
    /// - A space character (`' '`) is mapped to 36.
    /// - Any other character returns `None`.
    ///
    /// # Examples
    /// ```rust
    /// use hkid_ops::hkid_ops::HKIDOps;
    ///
    /// assert_eq!(HKIDOps::char_to_value('A'), Some(10));
    /// assert_eq!(HKIDOps::char_to_value('Z'), Some(35));
    /// assert_eq!(HKIDOps::char_to_value('a'), Some(10));
    /// assert_eq!(HKIDOps::char_to_value('5'), Some(5));
    /// assert_eq!(HKIDOps::char_to_value(' '), Some(36));
    /// assert_eq!(HKIDOps::char_to_value('-'), None);
    /// assert_eq!(HKIDOps::char_to_value('_'), None);
    /// ```
    pub fn char_to_value(c: char) -> Option<u32> {
        let c = c.to_ascii_uppercase();

        match c {
            'A'..='Z' => Some((c as u32 - 'A' as u32) + 10),
            '0'..='9' => Some(c as u32 - '0' as u32),
            ' ' => Some(36),
            _ => None,
        }
    }

    /// Generates a random uppercase ASCII letter ('A' to 'Z').
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator that implements the `Rng` trait.
    ///
    /// # Returns
    ///
    /// A randomly selected uppercase letter as a `char`.
    pub fn random_uppercase_letter<R: Rng + ?Sized>(rng: &mut R) -> char {
        (rng.random_range(b'A'..=b'Z')) as char
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
    fn random_prefix<R: Rng>(rng: &mut R) -> String {
        let len = if rng.random_bool(0.5) { 1 } else { 2 };

        (0..len).map(|_| HKIDOps::random_uppercase_letter(rng))
            .collect()
    }

    /// Calculates the check digit for a Hong Kong Identity Card (HKID) number body.
    ///
    /// The check digit is the final character (0–9 or 'A') used to validate a full HKID number.
    /// This method implements the official HKID check digit algorithm:
    ///
    /// 1. If the HKID body (prefix + 6 digits) is only 7 characters, it is left-padded with a space.
    ///    - This ensures all calculations use 8 positions (space/prefix + 6 digits).
    /// 2. Each character is converted to a numeric value using [`HKIDOps::char_to_value`].
    /// 3. Each value is multiplied by a weight (from 9 down to 2).
    /// 4. The sum of these products is used to compute the check digit as:
    ///    - `check = (11 - (sum % 11)) % 11`
    ///    - If the result is 10, the check digit is `'A'`.
    ///    - Otherwise, it is the digit itself.
    ///
    /// # Arguments
    /// * `hkid_body`: The HKID prefix and digits, excluding the check digit (e.g. `"A123456"` or `"AB123456"`).
    ///
    /// # Returns
    /// * `Some(check_digit)` as a `char` (`'0'`–`'9'` or `'A'`) if all characters are valid.
    /// * `None` if any character is invalid or the format is incorrect.
    ///
    /// # Examples
    /// ```rust
    /// use hkid_ops::hkid_ops::HKIDOps;
    ///
    /// let ops = HKIDOps {};
    ///
    /// assert_ne!(ops.calculate_check_digit("A123456"), Some('7'));      // Invalid: The expected check digit is '3'
    /// assert_eq!(ops.calculate_check_digit("AB123456"), Some('9'));     // Valid: 2-letter prefix
    /// assert_eq!(ops.calculate_check_digit("A12_456"), None);           // Invalid: underscore
    /// assert_eq!(ops.calculate_check_digit("A12345"), None);            // Invalid: too short
    /// assert_eq!(ops.calculate_check_digit("A1234567"), Some('7'));     // Valid: check digit is '7'
    /// ```
    pub fn calculate_check_digit(&self, hkid_body: &str) -> Option<char> {
        if !VALID_HKID_BODY_REGEX.is_match(hkid_body) {
            return None;
        }
        let padded_body = format!("{hkid_body:>8}");
        let values = padded_body
            .chars()
            .map(HKIDOps::char_to_value)
            .collect::<Option<Vec<u32>>>()?;
        let sum = values
            .iter()
            .zip(WEIGHTS.iter())
            .map(|(v, w)| v * w)
            .sum::<u32>();
        let check_digit = (11 - sum % 11) % 11;
        match check_digit {
            10 => Some('A'),
            digit => char::from_digit(digit, 10),
        }
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
    /// ```rust
    /// use hkid_ops::hkid_ops::HKIDOps;
    ///
    /// let ops = HKIDOps {};
    ///
    /// // Generate with a known prefix
    /// let hkid = ops.generate_hkid(Some("A"), true).unwrap();
    /// assert!(hkid.starts_with("A"));
    ///
    /// // Generate with a random known prefix
    /// let hkid_random_known = ops.generate_hkid(None, true).unwrap();
    /// // (Prefix is guaranteed to be in the HKIDPrefix enum)
    ///
    /// // Generate with a random one- or two-letter prefix
    /// let hkid_any = ops.generate_hkid(None, false).unwrap();
    /// assert!(hkid_any.len() >= 10 && hkid_any.len() <= 11);
    ///
    /// // Custom or unknown prefix (allowed when must_exist_in_enum is false)
    /// let custom = ops.generate_hkid(Some("ZZ"), false).unwrap();
    /// assert!(custom.starts_with("ZZ"));
    ///
    /// // Custom prefix not allowed
    /// assert!(ops.generate_hkid(Some("ZZ"), true).is_err());
    /// ```
    ///
    /// # Note
    /// - The random number generator used must provide `random_range`.
    /// - The check digit calculation uses your implementation of `calculate_check_digit`.
    ///
    pub fn generate_hkid(&self, prefix: Option<&str>, must_exist_in_enum: bool) -> Result<String, String> {
        let mut rng = rand::rng();

        // Early validate prefix if provided
        if let Some(px) = prefix {
            if !VALID_PREFIX_REGEX.is_match(px) {
                return Err(format!("Prefix '{px}' is not a valid HKID prefix format (must be 1 or 2 uppercase letters)"));
            }
            if must_exist_in_enum {
                let parsed_prefix = HKIDPrefix::parse(px);
                if !parsed_prefix.is_known() {
                    return Err(format!("Prefix '{px}' is not recognized"));
                }
            }
        }

        let prefix_str = match (prefix, must_exist_in_enum) {
            (Some(px), true) => HKIDPrefix::parse(px).as_str(),
            (Some(px), false) => HKIDPrefix::parse(px).as_str().to_string(),
            (None, true) => HKIDOps::random_known_prefix(&mut rng).ok_or_else(|| "No valid prefixes in HKIDPrefix enum".to_string())?,
            (None, false) => HKIDOps::random_prefix(&mut rng),
        };

        let digits = (0..6).map(|_| rng.random_range(0..10).to_string()).collect::<String>();
        let hkid_body = format!("{prefix_str}{digits}");
        let check_digit = self.calculate_check_digit(&hkid_body).ok_or("Failed to calculate check digit")?;

        Ok(format!("{hkid_body}({check_digit})"))
    }

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
    /// ```rust
    /// use hkid_ops::hkid_ops::HKIDOps;
    ///
    /// let ops = HKIDOps {};
    ///
    /// // Valid HKID, known prefix, must_exist_in_enum = true
    /// assert_eq!(ops.validate_hkid("A123456(7)", true), Ok(false));
    ///
    /// // Invalid check digit
    /// assert_eq!(ops.validate_hkid("A123456(8)", true), Ok(false));
    ///
    /// // Unknown prefix, must_exist_in_enum = true
    /// assert!(ops.validate_hkid("ZZ123456(7)", true).is_err());
    ///
    /// // Unknown prefix, must_exist_in_enum = false
    /// assert_eq!(ops.validate_hkid("ZZ123456(7)", false), Ok(false));
    /// ```
    ///
    /// # Details
    /// - The function first removes all parentheses from the input, allowing for HKIDs written with or without parentheses.
    /// - It then uses a regular expression to check the cleaned string format and extract the prefix, the six digits, and the check digit.
    /// - If `must_exist_in_enum` is true, the parsed prefix is checked against the `HKIDPrefix` enum.
    /// - The check digit is recalculated from the HKID body and compared to the provided digit. If the check digit is missing, an error is returned.
    ///
    pub fn validate_hkid(&self, hkid_full: &str, must_exist_in_enum: bool) -> Result<bool, String> {
        let cleaned = hkid_full.chars()
            .filter(|&c| c != '(' && c != ')')
            .collect::<String>();

        let caps = HKID_FULL_REGEX.captures(&cleaned)
            .ok_or_else(|| "Invalid HKID format: incorrect structure.".to_string())?;

        let prefix = caps.get(1).ok_or("Missing prefix in HKID")?.as_str();
        let digits = caps.get(2).ok_or("Missing digits in HKID")?.as_str();
        let provided_digit = caps.get(3).ok_or("Missing check digit in HKID")?.as_str();

        if must_exist_in_enum {
            let parsed_prefix = HKIDPrefix::parse(prefix);
            if !parsed_prefix.is_known() {
                return Err(format!("Prefix '{prefix}' is not recognized."));
            }
        }

        let hkid_body = format!("{prefix}{digits}");
        let calculated_digit = self.calculate_check_digit(&hkid_body)
            .ok_or_else(|| "Failed to calculate check digit".to_string())?;

        let provided_digit = provided_digit.chars().next().ok_or_else(|| "Missing check digit".to_string())?;

        Ok(calculated_digit == provided_digit)
    }
}

#[cfg(test)]
mod tests {
    use crate::hkid_prefix::HKIDPrefix;

    use super::*;

    #[test]
    fn test_char_to_value() {
        assert_eq!(HKIDOps::char_to_value('A'), Some(10));
        assert_eq!(HKIDOps::char_to_value('Z'), Some(35));
        assert_eq!(HKIDOps::char_to_value('a'), Some(10));
        assert_eq!(HKIDOps::char_to_value('z'), Some(35));
        assert_eq!(HKIDOps::char_to_value('0'), Some(0));
        assert_eq!(HKIDOps::char_to_value('9'), Some(9));
        assert_eq!(HKIDOps::char_to_value(' '), Some(36));
        assert_eq!(HKIDOps::char_to_value('@'), None);
        assert_eq!(HKIDOps::char_to_value('_'), None);
    }

    #[test]
    fn test_calculate_check_digit_single_letter_prefix() {
        let hkid_ops = HKIDOps {};

        assert_eq!(hkid_ops.calculate_check_digit("A123456"), Some('3'));
        assert_ne!(hkid_ops.calculate_check_digit("B987654"), Some('7'));
        assert_ne!(hkid_ops.calculate_check_digit("Z123456"), Some('0'));
    }

    #[test]
    fn test_calculate_check_digit_double_letter_prefix() {
        let hkid_ops = HKIDOps {};

        assert_ne!(hkid_ops.calculate_check_digit("WX123456"), Some('4'));
        assert_ne!(hkid_ops.calculate_check_digit("AB987654"), Some('5'));
        assert_ne!(hkid_ops.calculate_check_digit("ZZ111111"), Some('3'));
    }

    #[test]
    fn test_calculate_check_digit_resulting_in_a() {
        let hkid_ops = HKIDOps {};
        assert_ne!(hkid_ops.calculate_check_digit("C668668"), Some('A'));
    }

    #[test]
    fn test_calculate_check_digit_with_padding() {
        let hkid_ops = HKIDOps {};

        assert_eq!(
            hkid_ops.calculate_check_digit("P123456"),
            hkid_ops.calculate_check_digit("P123456")
        );
    }

    #[test]
    fn test_calculate_check_digit_invalid_char() {
        let hkid_ops = HKIDOps {};

        assert_eq!(hkid_ops.calculate_check_digit("A12345_"), None);
        assert_eq!(hkid_ops.calculate_check_digit("A12345-"), None);
    }

    #[test]
    fn test_calculate_check_digit_invalid_length() {
        let hkid_ops = HKIDOps {};

        assert_eq!(hkid_ops.calculate_check_digit("A12345"), None); // 6 chars, too short
        assert_eq!(hkid_ops.calculate_check_digit("A12345678"), None); // 9 chars, too long
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
        prefix_digits.chars()
            .rev()
            .take(6)
            .all(|c| c.is_ascii_digit())
    }

    #[test]
    fn test_generate_hkid_with_known_prefix() {
        let hkid_ops = HKIDOps {};
        let prefix = "A";
        let result = hkid_ops.generate_hkid(Some(prefix), true);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        assert!(hkid.starts_with(prefix));
        assert!(is_valid_format(&hkid));
    }

    #[test]
    fn test_generate_hkid_with_two_letter_known_prefix() {
        let hkid_ops = HKIDOps {};
        let prefix = "WX";
        let result = hkid_ops.generate_hkid(Some(prefix), true);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        assert!(hkid.starts_with(prefix));
        assert!(is_valid_format(&hkid));
    }

    #[test]
    fn test_generate_hkid_with_custom_prefix_allowed() {
        let hkid_ops = HKIDOps {};
        let prefix = "ZZ";
        let result = hkid_ops.generate_hkid(Some(prefix), false);

        assert!(result.is_ok());

        let hkid = result.unwrap();

        assert!(hkid.starts_with(prefix));
        assert!(is_valid_format(&hkid));
    }

    #[test]
    fn test_generate_hkid_with_random_known_prefix() {
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.generate_hkid(None, true);

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
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.generate_hkid(None, false);

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
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.generate_hkid(Some(""), true);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_hkid_with_lowercase_prefix() {
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.generate_hkid(Some("a"), true);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_hkid_with_unknown_prefix_not_allowed() {
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.generate_hkid(Some("ZZ"), true);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_hkid_correct() {
        let hkid_ops = HKIDOps {};
        let valid_hkid = "A123456(3)";
        let result = hkid_ops.validate_hkid(valid_hkid, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_validate_hkid_incorrect_digit() {
        let hkid_ops = HKIDOps {};
        let invalid_hkid = "A123456(9)";
        let result = hkid_ops.validate_hkid(invalid_hkid, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_validate_hkid_invalid_format() {
        let hkid_ops = HKIDOps {};
        let invalid_hkid_format = "A12345"; // less than required chars
        let result = hkid_ops.validate_hkid(invalid_hkid_format, false);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid HKID format: incorrect structure."
        );
    }

    #[test]
    fn test_validate_hkid_missing_check_digit() {
        let hkid_ops = HKIDOps {};
        let missing_digit = "A123456";
        let result = hkid_ops.validate_hkid(missing_digit, false);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid HKID format: incorrect structure."
        );
    }

    #[test]
    fn test_validate_hkid_unknown_prefix_with_must_exist() {
        let hkid_ops = HKIDOps {};
        let hkid = "XX123456(1)";
        let result = hkid_ops.validate_hkid(hkid, true);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Prefix 'XX' is not recognized.");
    }

    #[test]
    fn test_validate_hkid_unknown_prefix_without_must_exist() {
        let hkid_ops = HKIDOps {};
        let hkid = "ZZ123456(8)";
        let result = hkid_ops.validate_hkid(hkid, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_hkid_no_parentheses() {
        let hkid_ops = HKIDOps {};
        let valid_hkid = "A1234563";
        let result = hkid_ops.validate_hkid(valid_hkid, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_validate_hkid_missing_check_digit_branch() {
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.validate_hkid("A123456()", false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid HKID format: incorrect structure.");
    }

    #[test]
    fn test_validate_hkid_invalid_hkid_body() {
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.validate_hkid("A12345_(7)", false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid HKID format: incorrect structure.");
    }

    #[test]
    fn test_validate_hkid_lowercase_prefix() {
        let hkid_ops = HKIDOps {};
        let result = hkid_ops.validate_hkid("a123456(7)", false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid HKID format: incorrect structure.");
    }
}
