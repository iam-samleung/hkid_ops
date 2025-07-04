use crate::hkid_prefixes;

// This invokes the macro, expanding the enum and static variable
hkid_prefixes!(
    // Single-letter HKID prefixes
    A => "Original ID cards, issued between 1949 and 1962, most holders born before 1950",
    B => "Issued between 1955 and 1960 in city offices",
    C => "Issued between 1960 and 1983 in NT offices, mostly HK-born children (1946-1971)",
    D => "Issued between 1960 and 1983 at HK Island offices, mostly HK-born children",
    E => "Issued between 1955 and 1969 in Kowloon offices, mostly HK-born children (1946-1962)",
    F => "First issue of a card commencing from 24 February 2020",
    G => "Issued between 1967 and 1983 in Kowloon offices, children born 1956-1971",
    H => "Issued between 1979 and 1983 in HK Island offices, children born 1968-1971",
    J => "Consular officers",
    K => "First issue (1983 - 1990), children born 1972-1979",
    L => "Issued between 1983 and 2003 during computer malfunctions, very few holders",
    M => "First issue (2011 - 23 Feb 2020)",
    N => "Birth registered in Hong Kong after 1 June 2019",
    P => "First issue (1990 - 2000), children mostly born July-Dec 1979",
    R => "First issue (2000 - 2011)",
    S => "Birth registered in Hong Kong (1 Apr 2005 - 31 May 2019)",
    T => "Issued between 1983 and 1997 during computer malfunctions, very few holders",
    V => "Child under 11 issued \"Document of Identity for Visa Purposes\" (1983 - 2003)",
    W => "First issue to foreign laborer/domestic helper (10 Nov 1989 - 1 Jan 2009)",
    Y => "Birth registered in Hong Kong (1 Jan 1989 - 31 Mar 2005)",
    Z => "Birth registered in Hong Kong (1 Jan 1980 - 31 Dec 1988)",

    // Double-letter prefixes
    EC => "European Community officers and dependents (1993 - 2003)",
    WX => "Foreign laborers/domestic helpers issued since 2 Jan 2009",
    XA => "Persons without Chinese names issued before 27 Mar 1983",
    XB => "Persons without Chinese names issued before 27 Mar 1983",
    XC => "Persons without Chinese names issued before 27 Mar 1983",
    XD => "Persons without Chinese names issued before 27 Mar 1983",
    XE => "Persons without Chinese names issued before 27 Mar 1983",
    XG => "Persons without Chinese names issued before 27 Mar 1983",
    XH => "Persons without Chinese names issued before 27 Mar 1983"
);

impl HKIDPrefix {
    /// Attempts to parse a prefix string into its corresponding [`HKIDPrefix`] variant.
    ///
    /// If the input string matches a known HKID prefix (case-sensitive),
    /// the corresponding variant is returned. Otherwise, it returns
    /// [`HKIDPrefix::Unknown`] containing the original string.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hkid_ops::hkid_prefix::HKIDPrefix;
    ///
    /// assert_eq!(HKIDPrefix::parse("A"), HKIDPrefix::A);
    /// assert_eq!(HKIDPrefix::parse("EC"), HKIDPrefix::EC);
    /// assert_eq!(HKIDPrefix::parse("FOO"), HKIDPrefix::Unknown("FOO".to_string()));
    /// ```
    ///
    /// # Parameters
    /// - `prefix`: The prefix string to parse (e.g., "A", "EC").
    ///
    /// # Returns
    /// - A corresponding [`HKIDPrefix`] variant if recognized;
    /// - Otherwise, [`HKIDPrefix::Unknown`] with the original string.
    pub fn parse(prefix: &str) -> HKIDPrefix {
        prefix.parse().unwrap_or_else(|_| HKIDPrefix::Unknown(prefix.to_string()))
    }

    /// Returns the string representation of the HKID prefix.
    ///
    /// For known variants, this returns the debug format (e.g., "A", "EC").
    /// For `Unknown`, it returns the contained string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::hkid_ops::hkid_prefix::HKIDPrefix;
    ///
    /// assert_eq!(HKIDPrefix::A.as_str(), "A");
    /// assert_eq!(HKIDPrefix::Unknown("ZZ".to_string()).as_str(), "ZZ");
    /// ```
    pub fn as_str(&self) -> String {
        match self {
            HKIDPrefix::Unknown(s) => s.clone(),
            _ => format!("{self:?}"),
        }
    }

    /// Returns `true` if this prefix is a known, standard HKID prefix.
    ///
    /// Returns `false` if the prefix is `Unknown`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::hkid_ops::hkid_prefix::HKIDPrefix;
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
        // assert_eq!(prefix.get_message(), Some("Unknown or unspecified prefix"));
        assert_eq!(prefix.get_message(), None);
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

    #[test]
    fn test_parse_empty_string_and_lower() {
        let empty = HKIDPrefix::parse("");

        match empty {
            HKIDPrefix::Unknown(ref s) => assert_eq!(s, ""),
            _ => panic!("Expected Unknown"),
        }

        let lower = HKIDPrefix::parse("a");

        match lower {
            HKIDPrefix::Unknown(ref s) => assert_eq!(s, "a"),
            _ => panic!("Expected Unknown"),
        }

        let non_ascii = HKIDPrefix::parse("Ω");

        match non_ascii {
            HKIDPrefix::Unknown(ref s) => assert_eq!(s, "Ω"),
            _ => panic!("Expected Unknown"),
        }
    }

    #[test]
    fn test_as_str_and_is_known_all_variants() {
        use strum::IntoEnumIterator;

        for prefix in HKIDPrefix::iter() {
            let s = prefix.as_str();

            if let HKIDPrefix::Unknown(ref unk) = prefix {
                assert_eq!(s, unk.as_str());
                assert!(!prefix.is_known());
            } else {
                assert_eq!(s, format!("{:?}", prefix));
                assert!(prefix.is_known());
            }
        }

        let unknown = HKIDPrefix::Unknown("BAR".to_string());

        assert_eq!(unknown.as_str(), "BAR");
        assert!(!unknown.is_known());
    }
}
