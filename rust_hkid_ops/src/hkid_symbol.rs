use strum_macros::{EnumMessage, EnumProperty};

/// Represents the "symbol" or suffix found on Hong Kong Identity Cards (HKID).
///
/// Each symbol provides information about the cardholder's eligibility, rights, or card status,
/// such as eligibility for re-entry permits, right of abode, place of birth, issuing office, or lost card status.
///
/// ## Variants
/// - `AdultEligibleReentryPermit` (`***`): Holder is 18+ and eligible for Hong Kong Re-entry Permit.
/// - `YouthEligibleReentryPermit` (`*`): Holder is 11–17 and eligible for Re-entry Permit.
/// - `RightOfAbode` (`A`): Holder has the right of abode in Hong Kong.
/// - `BirthDateOrPlaceChanged` (`B`): Holder's reported date/place of birth changed since first registration.
/// - `StayLimitedByImmigration` (`C`): Holder's stay in Hong Kong is limited by the Director of Immigration.
/// - `NameChanged` (`N`): Holder's reported name has changed since first registration.
/// - `BornOutsideHKChinaMacau` (`O`): Holder was born outside Hong Kong, Mainland China, or Macau.
/// - `RightToLand` (`R`): Holder has the right to land in Hong Kong.
/// - `StayUnlimitedByImmigration` (`U`): Holder's stay is not limited by the Director of Immigration.
/// - `BornInMacau` (`W`): Holder's reported place of birth is Macau.
/// - `BornInMainlandChina` (`X`): Holder's reported place of birth is Mainland China.
/// - `BirthDateConfirmed` (`Y`): Holder's date of birth confirmed by certificate or passport.
/// - `BornInHongKong` (`Z`): Holder's reported place of birth is Hong Kong.
/// - `IssuingOfficeCode(String)`: Two-character code (e.g., `H1`, `K2`, `S1`), indicates the issuing office.
/// - `LostCard(u8)`: Card has been lost. `L1` for once, `L2` for twice, etc.
/// - `Unknown(String)`: Any unrecognized or custom symbol.
///
/// # Example
/// ```ignore
/// use hkid_ops::HKIDSymbol;
///
/// assert_eq!(HKIDSymbol::parse("***"), HKIDSymbol::AdultEligibleReentryPermit);
/// assert_eq!(HKIDSymbol::parse("A"), HKIDSymbol::RightOfAbode);
/// assert_eq!(HKIDSymbol::parse("H1"), HKIDSymbol::IssuingOfficeCode("H1".to_string()));
/// assert_eq!(HKIDSymbol::parse("L2"), HKIDSymbol::LostCard(2));
/// assert_eq!(HKIDSymbol::parse("QX"), HKIDSymbol::Unknown("QX".to_string()));
/// ```
#[derive(Debug, PartialEq, EnumMessage, EnumProperty)]
pub enum HKIDSymbol {
    #[strum(
        message = "The holder is aged 18 or over and eligible for a Hong Kong Re-entry Permit",
        props(Symbol = "***")
    )]
    AdultEligibleReentryPermit,

    #[strum(
        message = "The holder is aged between 11 and 17 and eligible for a Hong Kong Re-entry Permit",
        props(Symbol = "*")
    )]
    YouthEligibleReentryPermit,

    #[strum(
        message = "The holder has the right of abode in Hong Kong",
        props(Symbol = "A")
    )]
    RightOfAbode,

    #[strum(
        message = "The holder's reported date/place of birth has changed since first registration",
        props(Symbol = "B")
    )]
    BirthDateOrPlaceChanged,

    #[strum(
        message = "The holder's stay in Hong Kong is limited by the Director of Immigration at registration",
        props(Symbol = "C")
    )]
    StayLimitedByImmigration,

    #[strum(
        message = "The holder's reported name has changed since first registration",
        props(Symbol = "N")
    )]
    NameChanged,

    #[strum(
        message = "The holder was born outside Hong Kong, Mainland China, or Macau",
        props(Symbol = "O")
    )]
    BornOutsideHKChinaMacau,

    #[strum(
        message = "The holder has the right to land in Hong Kong",
        props(Symbol = "R")
    )]
    RightToLand,

    #[strum(
        message = "The holder's stay in Hong Kong is not limited by the Director of Immigration",
        props(Symbol = "U")
    )]
    StayUnlimitedByImmigration,

    #[strum(
        message = "The holder's reported place of birth is Macau",
        props(Symbol = "W")
    )]
    BornInMacau,

    #[strum(
        message = "The holder's reported place of birth is Mainland China",
        props(Symbol = "X")
    )]
    BornInMainlandChina,

    #[strum(
        message = "The holder's date of birth has been confirmed by birth certificate or passport",
        props(Symbol = "Y")
    )]
    BirthDateConfirmed,

    #[strum(
        message = "The holder's reported place of birth is Hong Kong",
        props(Symbol = "Z")
    )]
    BornInHongKong,

    #[strum(
        message = "Issuing office code (e.g., H1, K2, S1, P1, V1, etc.)",
        props(Symbol = "<Office Code>")
    )]
    IssuingOfficeCode(String),

    #[strum(
        message = "The holder has lost their ID card. 'L1' for once, 'L2' for twice, etc.",
        props(Symbol = "<L#>")
    )]
    LostCard(u8),

    #[strum(message = "Unknown or custom symbol", props(Symbol = "<Unknown>"))]
    Unknown(String),
}

impl HKIDSymbol {
    /// Parses a symbol string into an `HKIDSymbol` variant.
    ///
    /// Recognizes standard HKID symbols (`"A"`, `"B"`, `"***"`, `"L1"`, etc.), issuing office codes (two-character, e.g., `"H1"`),
    /// and lost card indicators (`"L1"`, `"L2"`, ...). Any unrecognized symbol is returned as `Unknown`.
    ///
    /// # Arguments
    /// * `symbol` - The symbol string to parse (e.g. `"A"`, `"***"`, `"H1"`, `"L3"`, `"ZZ"`).
    ///
    /// # Returns
    /// An appropriate `HKIDSymbol` variant.
    ///
    /// # Example
    /// ```ignore
    /// use hkid_ops::HKIDSymbol;
    ///
    /// assert_eq!(HKIDSymbol::parse("*"), HKIDSymbol::YouthEligibleReentryPermit);
    /// assert_eq!(HKIDSymbol::parse("L5"), HKIDSymbol::LostCard(5));
    /// assert_eq!(HKIDSymbol::parse("S1"), HKIDSymbol::IssuingOfficeCode("S1".to_string()));
    /// assert_eq!(HKIDSymbol::parse("??"), HKIDSymbol::Unknown("??".to_string()));
    /// ```
    pub fn parse(symbol: &str) -> HKIDSymbol {
        match symbol {
            "***" => HKIDSymbol::AdultEligibleReentryPermit,
            "*" => HKIDSymbol::YouthEligibleReentryPermit,
            "A" => HKIDSymbol::RightOfAbode,
            "B" => HKIDSymbol::BirthDateOrPlaceChanged,
            "C" => HKIDSymbol::StayLimitedByImmigration,
            "N" => HKIDSymbol::NameChanged,
            "O" => HKIDSymbol::BornOutsideHKChinaMacau,
            "R" => HKIDSymbol::RightToLand,
            "U" => HKIDSymbol::StayUnlimitedByImmigration,
            "W" => HKIDSymbol::BornInMacau,
            "X" => HKIDSymbol::BornInMainlandChina,
            "Y" => HKIDSymbol::BirthDateConfirmed,
            "Z" => HKIDSymbol::BornInHongKong,
            s if s.starts_with('L') && s.len() > 1 => {
                if let Ok(times) = s[1..].parse::<u8>() {
                    HKIDSymbol::LostCard(times)
                } else {
                    HKIDSymbol::Unknown(s.to_string())
                }
            }
            s if s.len() == 2 && s.chars().nth(1).unwrap().is_digit(10) => {
                HKIDSymbol::IssuingOfficeCode(s.to_string())
            }
            _ => HKIDSymbol::Unknown(symbol.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumMessage;
    use strum::EnumProperty;

    use super::*;

    #[test]
    fn test_symbol_and_message_basic_variants() {
        let symbol = HKIDSymbol::RightOfAbode;

        assert_eq!(symbol.get_str("Symbol"), Some("A"));
        assert_eq!(
            symbol.get_message(),
            Some("The holder has the right of abode in Hong Kong")
        );

        let symbol = HKIDSymbol::StayLimitedByImmigration;

        assert_eq!(symbol.get_str("Symbol"), Some("C"));
        assert_eq!(
            symbol.get_message(),
            Some("The holder's stay in Hong Kong is limited by the Director of Immigration at registration")
        );
    }

    #[test]
    fn test_symbol_and_message_lost_card() {
        let symbol = HKIDSymbol::LostCard(2);

        assert_eq!(symbol.get_str("Symbol"), Some("<L#>"));
        assert_eq!(
            symbol.get_message(),
            Some("The holder has lost their ID card. 'L1' for once, 'L2' for twice, etc.")
        );
    }

    #[test]
    fn test_symbol_and_message_issuing_office_code() {
        let symbol = HKIDSymbol::IssuingOfficeCode("K2".to_string());
        assert_eq!(symbol.get_str("Symbol"), Some("<Office Code>"));
        assert_eq!(
            symbol.get_message(),
            Some("Issuing office code (e.g., H1, K2, S1, P1, V1, etc.)")
        );
    }

    #[test]
    fn test_symbol_and_message_unknown() {
        let symbol = HKIDSymbol::Unknown("XYZ".to_string());
        assert_eq!(symbol.get_str("Symbol"), Some("<Unknown>"));
        assert_eq!(symbol.get_message(), Some("Unknown or custom symbol"));
    }

    #[test]
    fn test_parse_standard_symbols() {
        assert_eq!(
            HKIDSymbol::parse("***"),
            HKIDSymbol::AdultEligibleReentryPermit
        );
        assert_eq!(
            HKIDSymbol::parse("*"),
            HKIDSymbol::YouthEligibleReentryPermit
        );
        assert_eq!(HKIDSymbol::parse("A"), HKIDSymbol::RightOfAbode);
        assert_eq!(HKIDSymbol::parse("B"), HKIDSymbol::BirthDateOrPlaceChanged);
        assert_eq!(HKIDSymbol::parse("C"), HKIDSymbol::StayLimitedByImmigration);
        assert_eq!(HKIDSymbol::parse("N"), HKIDSymbol::NameChanged);
        assert_eq!(HKIDSymbol::parse("O"), HKIDSymbol::BornOutsideHKChinaMacau);
        assert_eq!(HKIDSymbol::parse("R"), HKIDSymbol::RightToLand);
        assert_eq!(
            HKIDSymbol::parse("U"),
            HKIDSymbol::StayUnlimitedByImmigration
        );
        assert_eq!(HKIDSymbol::parse("W"), HKIDSymbol::BornInMacau);
        assert_eq!(HKIDSymbol::parse("X"), HKIDSymbol::BornInMainlandChina);
        assert_eq!(HKIDSymbol::parse("Y"), HKIDSymbol::BirthDateConfirmed);
        assert_eq!(HKIDSymbol::parse("Z"), HKIDSymbol::BornInHongKong);
    }

    #[test]
    fn test_parse_lost_card() {
        assert_eq!(HKIDSymbol::parse("L1"), HKIDSymbol::LostCard(1));
        assert_eq!(HKIDSymbol::parse("L2"), HKIDSymbol::LostCard(2));
        assert_eq!(HKIDSymbol::parse("L10"), HKIDSymbol::LostCard(10));
        // Invalid lost card (not a number after L)
        assert_eq!(
            HKIDSymbol::parse("Lab"),
            HKIDSymbol::Unknown("Lab".to_string())
        );
    }

    #[test]
    fn test_parse_issuing_office_code() {
        assert_eq!(
            HKIDSymbol::parse("H1"),
            HKIDSymbol::IssuingOfficeCode("H1".to_string())
        );
        assert_eq!(
            HKIDSymbol::parse("K2"),
            HKIDSymbol::IssuingOfficeCode("K2".to_string())
        );
        assert_eq!(
            HKIDSymbol::parse("S9"),
            HKIDSymbol::IssuingOfficeCode("S9".to_string())
        );
        // Should not be recognized as issuing office code if not two characters or second not a digit
        assert_eq!(
            HKIDSymbol::parse("HF"),
            HKIDSymbol::Unknown("HF".to_string())
        );
    }

    #[test]
    fn test_parse_unknown() {
        assert_eq!(
            HKIDSymbol::parse("XYZ"),
            HKIDSymbol::Unknown("XYZ".to_string())
        );
        assert_eq!(HKIDSymbol::parse(""), HKIDSymbol::Unknown("".to_string()));
        assert_eq!(HKIDSymbol::parse("Q"), HKIDSymbol::Unknown("Q".to_string()));
        assert_eq!(HKIDSymbol::parse("1"), HKIDSymbol::Unknown("1".to_string()));
    }
}
