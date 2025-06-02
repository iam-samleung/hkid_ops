/// Defines the `HKIDPrefix` enum and a static slice of known prefix strings.
///
/// # Usage
///
/// ```rust
///
/// use hkid_ops::hkid_prefixes;
///
/// hkid_prefixes!(
///     A => "Original ID cards, issued between 1949 and 1962, most holders born before 1950",
///     B => "Issued between 1955 and 1960 in city offices",
///     // ... add more as needed ...
/// );
/// ```
///
/// # Expands to
///
/// - An enum `HKIDPrefix` with one variant per prefix, plus a catch-all `Unknown(String)`
/// - `#[strum(message = "...")]` for each variant, accessible via [`strum::EnumMessage::get_message()`]
/// - All useful [`strum`] derives for parsing, iterating, etc.
/// - A static `KNOWN_PREFIXES: &[&str]` containing the string names of all defined prefixes
///
/// # Example
///
/// ```rust
/// use strum::EnumMessage;
/// use crate::hkid_ops::hkid_prefix::{HKIDPrefix, KNOWN_PREFIXES};
///
/// let prefix = HKIDPrefix::A;
///
/// assert_eq!(prefix.get_message(), Some("Original ID cards, issued between 1949 and 1962, most holders born before 1950"));
/// assert!(KNOWN_PREFIXES.contains(&"A"));
/// ```
#[macro_export]
macro_rules! hkid_prefixes {
    (
        $(
            $prefix:ident => $msg:expr
        ),* $(,)?
    ) => {
        #[doc = "Represents the prefix portion of a Hong Kong Identity Card (HKID) number."]
        #[doc = ""]
        #[doc = "HKID prefixes are one or two letter codes at the start of the HKID, used to indicate issuing office,"]
        #[doc = "type of card, or special status. This enum enumerates all known standard HKID prefixes (both single-letter and double-letter),"]
        #[doc = "as well as an `Unknown` variant for non-standard or unrecognized prefixes."]
        #[doc = ""]
        #[doc = "# Variants"]
        #[doc = ""]
        #[doc = "- **Single-letter prefixes:**"]
        #[doc = "  - `A`: Original ID cards, issued between 1949 and 1962, most holders born before 1950"]
        #[doc = "  - `B`: Issued between 1955 and 1960 in city offices"]
        #[doc = "  - `C`: Issued between 1960 and 1983 in NT offices, mostly HK-born children (1946-1971)"]
        #[doc = "  - `D`: Issued between 1960 and 1983 at HK Island offices, mostly HK-born children"]
        #[doc = "  - `E`: Issued between 1955 and 1969 in Kowloon offices, mostly HK-born children (1946-1962)"]
        #[doc = "  - `F`: First issue of a card commencing from 24 February 2020"]
        #[doc = "  - `G`: Issued between 1967 and 1983 in Kowloon offices, children born 1956-1971"]
        #[doc = "  - `H`: Issued between 1979 and 1983 in HK Island offices, children born 1968-1971"]
        #[doc = "  - `J`: Consular officers"]
        #[doc = "  - `K`: First issue (1983 - 1990), children born 1972-1979"]
        #[doc = "  - `L`: Issued between 1983 and 2003 during computer malfunctions, very few holders"]
        #[doc = "  - `M`: First issue (2011 - 23 Feb 2020)"]
        #[doc = "  - `N`: Birth registered in Hong Kong after 1 June 2019"]
        #[doc = "  - `P`: First issue (1990 - 2000), children mostly born July-Dec 1979"]
        #[doc = "  - `R`: First issue (2000 - 2011)"]
        #[doc = "  - `S`: Birth registered in Hong Kong (1 Apr 2005 - 31 May 2019)"]
        #[doc = "  - `T`: Issued between 1983 and 1997 during computer malfunctions, very few holders"]
        #[doc = "  - `V`: Child under 11 issued \"Document of Identity for Visa Purposes\" (1983 - 2003)"]
        #[doc = "  - `W`: First issue to foreign laborer/domestic helper (10 Nov 1989 - 1 Jan 2009)"]
        #[doc = "  - `Y`: Birth registered in Hong Kong (1 Jan 1989 - 31 Mar 2005)"]
        #[doc = "  - `Z`: Birth registered in Hong Kong (1 Jan 1980 - 31 Dec 1988)"]
        #[doc = ""]
        #[doc = "- **Double-letter prefixes:**"]
        #[doc = "  - `EC`: European Community officers and dependents (1993 - 2003)"]
        #[doc = "  - `WX`: Foreign laborers/domestic helpers issued since 2 Jan 2009"]
        #[doc = "  - `XA`, `XB`, `XC`, `XD`, `XE`, `XG`, `XH`: Various special cases, often for persons without Chinese names issued before 27 Mar 1983"]
        #[doc = ""]
        #[doc = "- `Unknown(String)`: Any unrecognized or custom prefix not covered by the above variants."]
        #[doc = ""]
        #[doc = "# Example"]
        #[doc = "```"]
        #[doc = "use crate::hkid_ops::hkid_prefix::HKIDPrefix;"]
        #[doc = "let prefix = HKIDPrefix::parse(\"A\");"]
        #[doc = "assert_eq!(prefix, HKIDPrefix::A);"]
        #[doc = "let unknown = HKIDPrefix::parse(\"ZZ\");"]
        #[doc = "assert!(matches!(unknown, HKIDPrefix::Unknown(_)));"]
        #[doc = "```"]
        #[doc = ""]
        #[doc = "# See also"]
        #[doc = "- [`HKIDPrefix::parse`] for parsing a string to an `HKIDPrefix`."]
        #[doc = "- [`HKIDPrefix::as_str`] for obtaining a string representation."]
        #[doc = "- [`HKIDPrefix::is_known`] for checking if the prefix is recognized."]
        #[derive(
            Debug, PartialEq, Eq, Clone,
            strum_macros::EnumString,
            strum_macros::EnumMessage,
            strum_macros::AsRefStr,
            strum_macros::EnumIter,
            strum_macros::VariantNames,
            strum_macros::EnumProperty
        )]
        pub enum HKIDPrefix {
            $(
                #[strum(message = $msg)]
                $prefix,
            )*
            #[strum(default, disabled, message = "Unknown or unspecified prefix")]
            Unknown(String),
        }

        /// List of all defined HKID prefix strings (as `&str`).
        pub static KNOWN_PREFIXES: &[&str] = &[
            $( stringify!($prefix), )*
        ];
    }
}