# HKID Generator & Validator

A Rust library for generating and validating Hong Kong Identity Card (HKID) numbers, supporting all official and custom prefixes, with correct check digit calculation.

[![Build Status](https://img.shields.io/github/actions/workflow/status/iam-samleung/hkid_ops/master.yml?branch=master)](https://github.com/iam-samleung/hkid_ops)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/hkid_ops.svg)](https://crates.io/crates/hkid_ops)
[![codecov](https://img.shields.io/codecov/c/github/iam-samleung/hkid_ops/master?style=flat-square&logo=codecov)](https://codecov.io/gh/iam-samleung/hkid_ops)

## References

The implementation in this library is based on the following public resources:
 
- **HKID Check Digit Validation**
  - [Validation Rules - Hong Kong (SAP Help Portal)](https://help.sap.com/docs/successfactors-employee-central/countryregion-specifics/validation-rules-hong-kong)

- **HKID Symbol and Prefix Descriptions**
  - [Hong Kong identity card - Wikipedia](https://en.wikipedia.org/wiki/Hong_Kong_identity_card)

This ensures that the check digit calculation and the interpretation of HKID symbols and prefixes are consistent with established and publicly recognized standards.

---

## Features

- Parse HKID symbols and prefixes
- Generate valid HKIDs (with correct check digit)
- Validate HKID numbers (including check digit computation)
- Support for both known and unknown HKID prefixes

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hkid_ops = "0.1.3"
```

---

## Usage

```rust
use hkid_ops::{hkid_generator::generate_hkid, hkid_validator::validate_hkid};

fn main() {
    // Generate a random HKID with a known prefix
    let hkid = generate_hkid(None, true).unwrap();
    println!("Random valid HKID: {}", hkid);

    // Generate with a specific prefix
    let hkid_a = generate_hkid(Some("A"), true).unwrap();
    println!("HKID with prefix A: {}", hkid_a);

    // Generate with a custom (unknown) prefix
    let hkid_custom = generate_hkid(Some("ZZ"), false).unwrap();
    println!("HKID with custom prefix: {}", hkid_custom);

    // Validate an HKID
    match validate_hkid(&hkid_a, true) {
        Ok(valid) => println!("Is HKID valid? {}", valid),
        Err(e) => println!("Validation error: {}", e),
    }
}
```

---

## HKID Generation Logic

- **Known Prefix:**  
  When `must_exist_in_enum` is `true`, only standard HKID prefixes are allowed.
- **Random Prefix:**  
  When `prefix` is `None`:
    - If `must_exist_in_enum` is `true`, a random known prefix is chosen.
    - If `must_exist_in_enum` is `false`, a random 1- or 2-letter uppercase prefix is generated.
- **Unknown/Custom Prefix:**  
  Allowed only if `must_exist_in_enum` is `false`.

---
## License

This project is licensed under the [MIT License](./LICENSE).

You are free to use, modify, and distribute this software under the terms of the MIT License.
For detailed terms and conditions, please refer to the [LICENSE](./LICENSE) file in this repository.