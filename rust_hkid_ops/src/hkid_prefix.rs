use strum_macros::{AsRefStr, EnumIter, EnumMessage, EnumProperty, EnumString};

/// Represents the prefix portion of a Hong Kong Identity Card (HKID) number.
///
/// HKID prefixes are one or two letter codes at the start of the HKID, used to indicate issuing office,
/// type of card, or special status. This enum enumerates all known standard HKID prefixes (both single-letter and double-letter),
/// as well as an `Unknown` variant for non-standard or unrecognized prefixes.
///
/// # Variants
///
/// - **Single-letter prefixes:**
///   - `A`: Original ID cards, issued between 1949 and 1962, most holders born before 1950
///   - `B`: Issued between 1955 and 1960 in city offices
///   - `C`: Issued between 1960 and 1983 in NT offices, mostly HK-born children (1946-1971)
///   - `D`: Issued between 1960 and 1983 at HK Island offices, mostly HK-born children
///   - `E`: Issued between 1955 and 1969 in Kowloon offices, mostly HK-born children (1946-1962)
///   - `F`: First issue of a card commencing from 24 February 2020
///   - `G`: Issued between 1967 and 1983 in Kowloon offices, children born 1956-1971
///   - `H`: Issued between 1979 and 1983 in HK Island offices, children born 1968-1971
///   - `J`: Consular officers
///   - `K`: First issue (1983 - 1990), children born 1972-1979
///   - `L`: Issued between 1983 and 2003 during computer malfunctions, very few holders
///   - `M`: First issue (2011 - 23 Feb 2020)
///   - `N`: Birth registered in Hong Kong after 1 June 2019
///   - `P`: First issue (1990 - 2000), children mostly born July-Dec 1979
///   - `R`: First issue (2000 - 2011)
///   - `S`: Birth registered in Hong Kong (1 Apr 2005 - 31 May 2019)
///   - `T`: Issued between 1983 and 1997 during computer malfunctions, very few holders
///   - `V`: Child under 11 issued "Document of Identity for Visa Purposes" (1983 - 2003)
///   - `W`: First issue to foreign laborer/domestic helper (10 Nov 1989 - 1 Jan 2009)
///   - `Y`: Birth registered in Hong Kong (1 Jan 1989 - 31 Mar 2005)
///   - `Z`: Birth registered in Hong Kong (1 Jan 1980 - 31 Dec 1988)
///
/// - **Double-letter prefixes:**
///   - `EC`: European Community officers and dependents (1993 - 2003)
///   - `WX`: Foreign laborers/domestic helpers issued since 2 Jan 2009
///   - `XA`, `XB`, `XC`, `XD`, `XE`, `XG`, `XH`: Various special cases, often for persons without Chinese names issued before 27 Mar 1983
///
/// - `Unknown(String)`: Any unrecognized or custom prefix not covered by the above variants.
///
/// # Example
/// ```ignore
/// use hkid_ops::utils::hkid_prefix::HKIDPrefix;
///
/// let prefix = HKIDPrefix::parse("A");
/// assert_eq!(prefix, HKIDPrefix::A);
///
/// let unknown = HKIDPrefix::parse("ZZ");
/// assert!(matches!(unknown, HKIDPrefix::Unknown(_)));
/// ```
///
/// # See also
/// - [`HKIDPrefix::parse`] for parsing a string to an `HKIDPrefix`.
/// - [`HKIDPrefix::as_str`] for obtaining a string representation.
/// - [`HKIDPrefix::is_known`] for checking if the prefix is recognized.
///
#[derive(Debug, PartialEq, EnumString, EnumMessage, AsRefStr, EnumProperty, EnumIter)]
pub enum HKIDPrefix {
    // Single-letter HKID prefixes
    #[strum(
        message = "Original ID cards, issued between 1949 and 1962, most holders born before 1950"
    )]
    A,
    #[strum(message = "Issued between 1955 and 1960 in city offices")]
    B,
    #[strum(
        message = "Issued between 1960 and 1983 in NT offices, mostly HK-born children (1946-1971)"
    )]
    C,
    #[strum(
        message = "Issued between 1960 and 1983 at HK Island offices, mostly HK-born children"
    )]
    D,
    #[strum(
        message = "Issued between 1955 and 1969 in Kowloon offices, mostly HK-born children (1946-1962)"
    )]
    E,
    #[strum(message = "First issue of a card commencing from 24 February 2020")]
    F,
    #[strum(message = "Issued between 1967 and 1983 in Kowloon offices, children born 1956-1971")]
    G,
    #[strum(
        message = "Issued between 1979 and 1983 in HK Island offices, children born 1968-1971"
    )]
    H,
    #[strum(message = "Consular officers")]
    J,
    #[strum(message = "First issue (1983 - 1990), children born 1972-1979")]
    K,
    #[strum(
        message = "Issued between 1983 and 2003 during computer malfunctions, very few holders"
    )]
    L,
    #[strum(message = "First issue (2011 - 23 Feb 2020)")]
    M,
    #[strum(message = "Birth registered in Hong Kong after 1 June 2019")]
    N,
    #[strum(message = "First issue (1990 - 2000), children mostly born July-Dec 1979")]
    P,
    #[strum(message = "First issue (2000 - 2011)")]
    R,
    #[strum(message = "Birth registered in Hong Kong (1 Apr 2005 - 31 May 2019)")]
    S,
    #[strum(
        message = "Issued between 1983 and 1997 during computer malfunctions, very few holders"
    )]
    T,
    #[strum(
        message = "Child under 11 issued \"Document of Identity for Visa Purposes\" (1983 - 2003)"
    )]
    V,
    #[strum(message = "First issue to foreign laborer/domestic helper (10 Nov 1989 - 1 Jan 2009)")]
    W,
    #[strum(message = "Birth registered in Hong Kong (1 Jan 1989 - 31 Mar 2005)")]
    Y,
    #[strum(message = "Birth registered in Hong Kong (1 Jan 1980 - 31 Dec 1988)")]
    Z,

    // Double-letter prefixes
    #[strum(message = "European Community officers and dependents (1993 - 2003)")]
    EC,
    #[strum(message = "Foreign laborers/domestic helpers issued since 2 Jan 2009")]
    WX,
    #[strum(message = "Persons without Chinese names issued before 27 Mar 1983")]
    XA,
    #[strum(message = "Persons without Chinese names issued before 27 Mar 1983")]
    XB,
    #[strum(message = "Persons without Chinese names issued before 27 Mar 1983")]
    XC,
    #[strum(message = "Persons without Chinese names issued before 27 Mar 1983")]
    XD,
    #[strum(message = "Persons without Chinese names issued before 27 Mar 1983")]
    XE,
    #[strum(message = "Persons without Chinese names issued before 27 Mar 1983")]
    XG,
    #[strum(message = "Persons without Chinese names issued before 27 Mar 1983")]
    XH,

    #[strum(message = "Unknown or unspecified prefix")]
    Unknown(String),
}

impl HKIDPrefix {
    /// Parses a prefix string into the corresponding HKIDPrefix enum variant.
    pub fn parse(prefix: &str) -> HKIDPrefix {
        match prefix {
            "A" => HKIDPrefix::A,
            "B" => HKIDPrefix::B,
            "C" => HKIDPrefix::C,
            "D" => HKIDPrefix::D,
            "E" => HKIDPrefix::E,
            "F" => HKIDPrefix::F,
            "G" => HKIDPrefix::G,
            "H" => HKIDPrefix::H,
            "J" => HKIDPrefix::J,
            "K" => HKIDPrefix::K,
            "L" => HKIDPrefix::L,
            "M" => HKIDPrefix::M,
            "N" => HKIDPrefix::N,
            "P" => HKIDPrefix::P,
            "R" => HKIDPrefix::R,
            "S" => HKIDPrefix::S,
            "T" => HKIDPrefix::T,
            "V" => HKIDPrefix::V,
            "W" => HKIDPrefix::W,
            "Y" => HKIDPrefix::Y,
            "Z" => HKIDPrefix::Z,
            "EC" => HKIDPrefix::EC,
            "WX" => HKIDPrefix::WX,
            "XA" => HKIDPrefix::XA,
            "XB" => HKIDPrefix::XB,
            "XC" => HKIDPrefix::XC,
            "XD" => HKIDPrefix::XD,
            "XE" => HKIDPrefix::XE,
            "XG" => HKIDPrefix::XG,
            "XH" => HKIDPrefix::XH,
            other => HKIDPrefix::Unknown(other.to_string()),
        }
    }

    /// Get the string representation of the prefix
    #[deprecated(since = "0.1.0", note = "please use `as_str` instead")]
    #[allow(dead_code)]
    pub fn to_str(&self) -> &str {
        match self {
            HKIDPrefix::A => "A",
            HKIDPrefix::B => "B",
            HKIDPrefix::C => "C",
            HKIDPrefix::D => "D",
            HKIDPrefix::E => "E",
            HKIDPrefix::F => "F",
            HKIDPrefix::G => "G",
            HKIDPrefix::H => "H",
            HKIDPrefix::J => "J",
            HKIDPrefix::K => "K",
            HKIDPrefix::L => "L",
            HKIDPrefix::M => "M",
            HKIDPrefix::N => "N",
            HKIDPrefix::P => "P",
            HKIDPrefix::R => "R",
            HKIDPrefix::S => "S",
            HKIDPrefix::T => "T",
            HKIDPrefix::V => "V",
            HKIDPrefix::W => "W",
            HKIDPrefix::Y => "Y",
            HKIDPrefix::Z => "Z",
            HKIDPrefix::EC => "EC",
            HKIDPrefix::WX => "WX",
            HKIDPrefix::XA => "XA",
            HKIDPrefix::XB => "XB",
            HKIDPrefix::XC => "XC",
            HKIDPrefix::XD => "XD",
            HKIDPrefix::XE => "XE",
            HKIDPrefix::XG => "XG",
            HKIDPrefix::XH => "XH",
            HKIDPrefix::Unknown(s) => s,
        }
    }

    /// Returns the string representation of the HKID prefix.
    ///
    /// For known variants, this returns the debug format (e.g., "A", "EC").
    /// For `Unknown`, it returns the contained string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use hkid_ops::utils::hkid_prefix::HKIDPrefix;
    ///
    /// assert_eq!(HKIDPrefix::A.as_str(), "A");
    /// assert_eq!(HKIDPrefix::Unknown("ZZ".to_string()).as_str(), "ZZ");
    /// ```
    #[allow(dead_code)]
    pub fn as_str(&self) -> String {
        match self {
            HKIDPrefix::Unknown(s) => s.clone(),
            _ => format!("{:?}", self),
        }
    }

    /// Returns `true` if this prefix is a known, standard HKID prefix.
    ///
    /// Returns `false` if the prefix is `Unknown`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use hkid_ops::utils::hkid_prefix::HKIDPrefix;
    ///
    /// assert!(HKIDPrefix::A.is_known());
    /// assert!(!HKIDPrefix::Unknown("ZZ".to_string()).is_known());
    /// ```
    pub fn is_known(&self) -> bool {
        !matches!(self, HKIDPrefix::Unknown(_))
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumMessage;

    use super::*;

    #[test]
    fn test_code_and_message_basic_variant() {
        let prefix = HKIDPrefix::A;
        assert_eq!(
            prefix.get_message(),
            Some("Original ID cards, issued between 1949 and 1962, most holders born before 1950")
        );

        let prefix = HKIDPrefix::EC;
        assert_eq!(
            prefix.get_message(),
            Some("European Community officers and dependents (1993 - 2003)")
        );
    }

    #[test]
    fn test_code_and_message_xb_variant() {
        let prefix = HKIDPrefix::XB;
        assert_eq!(
            prefix.get_message(),
            Some("Persons without Chinese names issued before 27 Mar 1983")
        );
    }

    #[test]
    fn test_code_and_message_unknown_variant() {
        let prefix = HKIDPrefix::Unknown("FOO".to_string());
        assert_eq!(prefix.get_message(), Some("Unknown or unspecified prefix"));
    }

    #[test]
    fn test_parse_known_single_letter_prefixes() {
        assert_eq!(HKIDPrefix::parse("A"), HKIDPrefix::A);
        assert_eq!(HKIDPrefix::parse("B"), HKIDPrefix::B);
        assert_eq!(HKIDPrefix::parse("C"), HKIDPrefix::C);
        assert_eq!(HKIDPrefix::parse("D"), HKIDPrefix::D);
        assert_eq!(HKIDPrefix::parse("E"), HKIDPrefix::E);
        assert_eq!(HKIDPrefix::parse("F"), HKIDPrefix::F);
        assert_eq!(HKIDPrefix::parse("G"), HKIDPrefix::G);
        assert_eq!(HKIDPrefix::parse("H"), HKIDPrefix::H);
        assert_eq!(HKIDPrefix::parse("J"), HKIDPrefix::J);
        assert_eq!(HKIDPrefix::parse("K"), HKIDPrefix::K);
        assert_eq!(HKIDPrefix::parse("L"), HKIDPrefix::L);
        assert_eq!(HKIDPrefix::parse("M"), HKIDPrefix::M);
        assert_eq!(HKIDPrefix::parse("N"), HKIDPrefix::N);
        assert_eq!(HKIDPrefix::parse("P"), HKIDPrefix::P);
        assert_eq!(HKIDPrefix::parse("R"), HKIDPrefix::R);
        assert_eq!(HKIDPrefix::parse("S"), HKIDPrefix::S);
        assert_eq!(HKIDPrefix::parse("T"), HKIDPrefix::T);
        assert_eq!(HKIDPrefix::parse("V"), HKIDPrefix::V);
        assert_eq!(HKIDPrefix::parse("W"), HKIDPrefix::W);
        assert_eq!(HKIDPrefix::parse("Y"), HKIDPrefix::Y);
        assert_eq!(HKIDPrefix::parse("Z"), HKIDPrefix::Z);
    }

    #[test]
    fn test_parse_known_double_letter_prefixes() {
        assert_eq!(HKIDPrefix::parse("EC"), HKIDPrefix::EC);
        assert_eq!(HKIDPrefix::parse("WX"), HKIDPrefix::WX);
        assert_eq!(HKIDPrefix::parse("XA"), HKIDPrefix::XA);
        assert_eq!(HKIDPrefix::parse("XB"), HKIDPrefix::XB);
        assert_eq!(HKIDPrefix::parse("XC"), HKIDPrefix::XC);
        assert_eq!(HKIDPrefix::parse("XD"), HKIDPrefix::XD);
        assert_eq!(HKIDPrefix::parse("XE"), HKIDPrefix::XE);
        assert_eq!(HKIDPrefix::parse("XG"), HKIDPrefix::XG);
        assert_eq!(HKIDPrefix::parse("XH"), HKIDPrefix::XH);
    }

    #[test]
    fn test_parse_unknown_prefix() {
        let unknown = HKIDPrefix::parse("ZZ");
        match unknown {
            HKIDPrefix::Unknown(ref s) => assert_eq!(s, "ZZ"),
            _ => panic!("Expected Unknown variant"),
        }

        let unknown2 = HKIDPrefix::parse("random");
        match unknown2 {
            HKIDPrefix::Unknown(ref s) => assert_eq!(s, "random"),
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_to_str_known_prefixes() {
        let prefixes = vec![
            ("A", HKIDPrefix::A),
            ("B", HKIDPrefix::B),
            ("C", HKIDPrefix::C),
            ("D", HKIDPrefix::D),
            ("E", HKIDPrefix::E),
            ("F", HKIDPrefix::F),
            ("G", HKIDPrefix::G),
            ("H", HKIDPrefix::H),
            ("J", HKIDPrefix::J),
            ("K", HKIDPrefix::K),
            ("L", HKIDPrefix::L),
            ("M", HKIDPrefix::M),
            ("N", HKIDPrefix::N),
            ("P", HKIDPrefix::P),
            ("R", HKIDPrefix::R),
            ("S", HKIDPrefix::S),
            ("T", HKIDPrefix::T),
            ("V", HKIDPrefix::V),
            ("W", HKIDPrefix::W),
            ("Y", HKIDPrefix::Y),
            ("Z", HKIDPrefix::Z),
            ("EC", HKIDPrefix::EC),
            ("WX", HKIDPrefix::WX),
            ("XA", HKIDPrefix::XA),
            ("XB", HKIDPrefix::XB),
            ("XC", HKIDPrefix::XC),
            ("XD", HKIDPrefix::XD),
            ("XE", HKIDPrefix::XE),
            ("XG", HKIDPrefix::XG),
            ("XH", HKIDPrefix::XH),
        ];
        for (str_repr, variant) in prefixes {
            assert_eq!(variant.to_str(), str_repr);
        }
    }

    #[test]
    fn test_to_str_unknown_prefix() {
        let unknown = HKIDPrefix::Unknown("ZZ".to_string());
        assert_eq!(unknown.as_str(), "ZZ");
    }

    #[test]
    fn test_as_str_known_prefixes() {
        assert_eq!(HKIDPrefix::A.as_str(), "A");
        assert_eq!(HKIDPrefix::EC.as_str(), "EC");
        assert_eq!(HKIDPrefix::XH.as_str(), "XH");
    }

    #[test]
    fn test_as_str_unknown_prefix() {
        let unknown = HKIDPrefix::Unknown("FOO".to_string());
        assert_eq!(unknown.as_str(), "FOO");
    }

    #[test]
    fn test_is_known_for_known_and_unknown() {
        assert!(HKIDPrefix::A.is_known());
        assert!(HKIDPrefix::WX.is_known());
        assert!(!HKIDPrefix::Unknown("BAR".to_string()).is_known());
    }
}
