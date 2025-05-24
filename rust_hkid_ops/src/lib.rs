//! # `hkid_ops` Usage Examples
//!
//! This section demonstrates common usage patterns for the `hkid_ops` library, including symbol and prefix parsing, HKID generation, and validation.
//!
//! ---
//!
//! ## 1. HKID Symbol Parsing
//!
//! ```ignore
//! use hkid_ops::HKIDSymbol;
//!
//! let symbols = [
//!     "***", "*", "A", "B", "C", "N", "O", "R", "U", "W", "X", "Y", "Z", "H1", "L2", "Unknown"
//! ];
//! for (i, sym) in symbols.iter().enumerate() {
//!     let parsed = HKIDSymbol::parse(sym);
//!     println!("[{}] Input: {:?} => Parsed: {:?}", i + 1, sym, parsed);
//! }
//! ```
//!
//! ## 2. HKID Prefix Parsing
//!
//! ```ignore
//! use hkid_ops::HKIDPrefix;
//!
//! let prefixes = [
//!     "A", "C", "F", "K", "N", "R", "Z", "EC", "WX", "XA", "Unknown"
//! ];
//! for (i, prefix) in prefixes.iter().enumerate() {
//!     let parsed = HKIDPrefix::parse(prefix);
//!     println!("[{}] Input: {:?} => Parsed: {:?}", i + 1, prefix, parsed);
//! }
//! ```
//!
//! ## 3. HKID Generation and Validation (Known Prefixes)
//!
//! ```ignore
//! use hkid_ops::{hkid_generator, validate_hkid};
//!
//! let gen_prefixes = ["A", "K", "WX", "XA"];
//! for (i, prefix) in gen_prefixes.iter().enumerate() {
//!     println!("\n[{}] Generating HKID with prefix '{}':", i + 1, prefix);
//!     match hkid_generator::generate_hkid(Some(prefix), true) {
//!         Ok(hkid) => {
//!             println!("Generated: {}", hkid);
//!             // Validate the generated HKID
//!             match validate_hkid(&hkid, true) {
//!                 Ok(valid) => println!("    Validation result: {}", if valid { "Valid" } else { "Invalid" }),
//!                 Err(e) => println!("    Validation error: {}", e),
//!             }
//!         }
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ## 4. HKID Generation Allowing Unknown Prefixes
//!
//! ```ignore
//! use hkid_ops::hkid_generator;
//!
//! let test_prefixes = ["A", "WX", "ZZ"];
//! for prefix in test_prefixes {
//!     println!("\nGenerating HKID with prefix '{}', must_exist_in_enum = true:", prefix);
//!     match hkid_generator::generate_hkid(Some(prefix), true) {
//!         Ok(hkid) => println!("    Generated: {}", hkid),
//!         Err(e) => println!("    Error: {}", e),
//!     }
//!     println!("Generating HKID with prefix '{}', must_exist_in_enum = false:", prefix);
//!     match hkid_generator::generate_hkid(Some(prefix), false) {
//!         Ok(hkid) => println!("    Generated (allowed unknown): {}", hkid),
//!         Err(e) => println!("    Error: {}", e),
//!     }
//! }
//! ```
//!
//! ## 5. HKID Generation Using a Random Prefix
//!
//! ```ignore
//! use hkid_ops::{hkid_generator, validate_hkid};
//!
//! // Random known prefix
//! println!("Generating HKID with random prefix (must_exist_in_enum = true):");
//! match hkid_generator::generate_hkid(None, true) {
//!     Ok(hkid) => {
//!         println!("    Generated: {}", hkid);
//!         match validate_hkid(&hkid, true) {
//!             Ok(valid) => println!("    Validation result: {}", if valid { "Valid" } else { "Invalid" }),
//!             Err(e) => println!("    Validation error: {}", e),
//!         }
//!     }
//!     Err(e) => println!("    Error: {}", e),
//! }
//!
//! // Random unknown-or-known prefix
//! println!("Generating HKID with random prefix (must_exist_in_enum = false):");
//! match hkid_generator::generate_hkid(None, false) {
//!     Ok(hkid) => {
//!         println!("    Generated: {}", hkid);
//!         match validate_hkid(&hkid, false) {
//!             Ok(valid) => println!("    Validation result: {}", if valid { "Valid" } else { "Invalid" }),
//!             Err(e) => println!("    Validation error: {}", e),
//!         }
//!     }
//!     Err(e) => println!("    Error: {}", e),
//! }
//! ```
//!
//! ## 6. HKID Validation (Various Samples)
//!
//! ```ignore
//! use hkid_ops::validate_hkid;
//!
//! let samples = [
//!     ("A123456(7)", true),    // Valid, known prefix, correct format
//!     ("AB123456(9)", true),   // Valid, known prefix, correct format
//!     ("ZZ123456(9)", false),  // Unknown prefix, but allowed
//!     ("A123456(8)", true),    // Invalid check digit
//!     ("A12345(7)", true),     // Invalid length
//!     ("A123456(7)", false),   // Accept unknown prefix (should be fine)
//!     ("PB100001(8)", false),  // Valid for unknown prefix
//!     ("PB100001(8)", true),   // Unknown prefix, must_exist_in_enum
//! ];
//! for (i, (hkid, must_exist)) in samples.iter().enumerate() {
//!     println!("\n[{}] Validating HKID '{}', must_exist_in_enum = {}:", i + 1, hkid, must_exist);
//!     match validate_hkid(hkid, *must_exist) {
//!         Ok(valid) => println!("{}", if valid { "Valid" } else { "Invalid" }),
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ---
//!
//! For more advanced or interactive usage, see the `examples/` directory of the repository.
//!
//! > These code snippets are for demonstration purposes; some require the crate to be used as a binary for visible output via `println!`.
//!

pub mod hkid_check_digit;
pub mod hkid_generator;
pub mod hkid_prefix;
pub mod hkid_symbol;
pub mod hkid_validator;

use once_cell::sync::Lazy;
use regex::Regex;

const WEIGHTS: [u32; 8] = [9, 8, 7, 6, 5, 4, 3, 2];

/// Pattern for a valid HKID body: 7 or 8 uppercase letters/digits (A-Z, 0-9)
const VALID_HKID_BODY_PATTERN: &str = r"^[A-Z0-9]{7,8}$";
// For best efficiency (and to avoid recompiling the regex on every function call)
static VALID_HKID_BODY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(VALID_HKID_BODY_PATTERN).unwrap());

// Regex pattern for a full HKID:
// - 1 or 2 uppercase letters (prefix)
// - 6 digits
// - 1 check digit (A or 0-9)
const HKID_FULL_PATTERN: &str = r"^([A-Z]{1,2})([0-9]{6})([A0-9])$";
// Compiled regex for matching full HKID against its official structure.
static HKID_FULL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(HKID_FULL_PATTERN).unwrap());