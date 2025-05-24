//! # hkid_ops — HKID Toolkit
//!
//! This crate provides parsing, generation, and validation utilities for Hong Kong Identity Cards (HKID).
//!
//! ## HKID Check Digit Algorithm
//!
//! The HKID check digit is calculated using a weighted sum of the HKID body (prefix + 6 digits).
//! Each character is mapped to a numeric value (`A=10`, ..., `Z=35`, `0=0`, ..., `9=9`, space=`36`), then multiplied by a weight.
//!
//! The weights are defined in [`WEIGHTS`](crate::WEIGHTS):
//!
//! ```rust
//! pub const WEIGHTS: [u32; 8] = [9, 8, 7, 6, 5, 4, 3, 2];
//! ```
//!
//! - If the HKID body is 7 characters, it is left-padded with a space.
//! - Each character's value is multiplied by the corresponding weight.
//! - Add up the products, then compute the check digit as `(11 - (sum % 11)) % 11`.
//! - If the result is 10, the check digit is `'A'`; otherwise, it's the digit itself.
//!
//! ---
//!
//! ## Usage Examples
//!
//! ### 1. HKID Symbol Parsing
//!
//! ```rust
//! use hkid_ops::hkid_symbol::HKIDSymbol;
//!
//! let symbols = [
//!     "***", "*", "A", "B", "C", "N", "O", "R", "U", "W", "X", "Y", "Z", "H1", "L2", "Unknown"
//! ];
//! for sym in symbols.iter() {
//!     let parsed = HKIDSymbol::parse(sym);
//!     println!("Input: {:?} => Parsed: {:?}", sym, parsed);
//! }
//! ```
//!
//! ### 2. HKID Prefix Parsing
//!
//! ```rust
//! use hkid_ops::hkid_prefix::HKIDPrefix;
//!
//! let prefixes = [
//!     "A", "C", "F", "K", "N", "R", "Z", "EC", "WX", "XA", "Unknown"
//! ];
//! for prefix in prefixes.iter() {
//!     let parsed = HKIDPrefix::parse(prefix);
//!     println!("Input: {:?} => Parsed: {:?}", prefix, parsed);
//! }
//! ```
//!
//! ### 3. HKID Generation and Validation (Known Prefixes)
//!
//! ```rust
//! use hkid_ops::hkid_ops::HKIDOps;
//!
//! let ops = HKIDOps {};
//! let gen_prefixes = ["A", "K", "WX", "XA"];
//! for prefix in gen_prefixes.iter() {
//!     println!("\nGenerating HKID with prefix '{}':", prefix);
//!     match ops.generate_hkid(Some(prefix), true) {
//!         Ok(hkid) => {
//!             println!("Generated: {}", hkid);
//!             // Validate the generated HKID
//!             match ops.validate_hkid(&hkid, true) {
//!                 Ok(valid) => println!("    Validation result: {}", if valid { "Valid" } else { "Invalid" }),
//!                 Err(e) => println!("    Validation error: {}", e),
//!             }
//!         }
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ### 4. HKID Generation Allowing Unknown Prefixes
//!
//! ```rust
//! use hkid_ops::hkid_ops::HKIDOps;
//!
//! let ops = HKIDOps {};
//! let test_prefixes = ["A", "WX", "ZZ"];
//! for prefix in test_prefixes {
//!     println!("Generating HKID with prefix '{}', must_exist_in_enum = true:", prefix);
//!     match ops.generate_hkid(Some(prefix), true) {
//!         Ok(hkid) => println!("    Generated: {}", hkid),
//!         Err(e) => println!("    Error: {}", e),
//!     }
//!     println!("Generating HKID with prefix '{}', must_exist_in_enum = false:", prefix);
//!     match ops.generate_hkid(Some(prefix), false) {
//!         Ok(hkid) => println!("    Generated (allowed unknown): {}", hkid),
//!         Err(e) => println!("    Error: {}", e),
//!     }
//! }
//! ```
//!
//! ### 5. HKID Generation Using a Random Prefix
//!
//! ```rust
//! use hkid_ops::hkid_ops::HKIDOps;
//!
//! let ops = HKIDOps {};
//!
//! // Random known prefix
//! match ops.generate_hkid(None, true) {
//!     Ok(hkid) => {
//!         println!("Generated with random known prefix: {}", hkid);
//!         match ops.validate_hkid(&hkid, true) {
//!             Ok(valid) => println!("    Validation result: {}", if valid { "Valid" } else { "Invalid" }),
//!             Err(e) => println!("    Validation error: {}", e),
//!         }
//!     }
//!     Err(e) => println!("    Error: {}", e),
//! }
//!
//! // Random unknown-or-known prefix
//! match ops.generate_hkid(None, false) {
//!     Ok(hkid) => {
//!         println!("Generated with random any prefix: {}", hkid);
//!         match ops.validate_hkid(&hkid, false) {
//!             Ok(valid) => println!("    Validation result: {}", if valid { "Valid" } else { "Invalid" }),
//!             Err(e) => println!("    Validation error: {}", e),
//!         }
//!     }
//!     Err(e) => println!("    Error: {}", e),
//! }
//! ```
//!
//! ### 6. HKID Validation (Various Samples)
//!
//! ```rust
//! use hkid_ops::hkid_ops::HKIDOps;
//!
//! let ops = HKIDOps {};
//! let samples = [
//!     ("A123456(3)", true),    // Valid, known prefix, correct check digit
//!     ("AB123456(9)", true),   // Valid, known prefix, correct check digit
//!     ("ZZ123456(9)", false),  // Unknown prefix, but allowed
//!     ("A123456(8)", true),    // Invalid check digit
//!     ("A12345(7)", true),     // Invalid length
//!     ("A123456(3)", false),   // Accept unknown prefix (should be fine)
//!     ("PB100001(8)", false),  // Valid for unknown prefix
//!     ("PB100001(8)", true),   // Unknown prefix, must_exist_in_enum
//! ];
//! for (hkid, must_exist) in samples.iter() {
//!     println!("Validating HKID '{}', must_exist_in_enum = {}:", hkid, must_exist);
//!     match ops.validate_hkid(hkid, *must_exist) {
//!         Ok(valid) => println!("{}", if valid { "Valid" } else { "Invalid" }),
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```

pub mod hkid_prefix;
pub mod hkid_symbol;
pub mod hkid_ops;
