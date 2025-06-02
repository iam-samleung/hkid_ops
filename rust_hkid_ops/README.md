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

- **Mathematical Concepts Underlying Check Digit Algorithms: Weight Functions, ISBN Standards, and Modular Arithmetic**
  - [Weight function - Wikipedia](https://en.wikipedia.org/wiki/Weight_function)
  - [ISBN](https://en.wikipedia.org/wiki/ISBN)
  - [Modular arithmetic](https://en.wikipedia.org/wiki/Modular_arithmetic)

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
hkid_ops = "0.2.5"
```

---

## Usage

```rust
use hkid_ops::hkid_ops::HKIDOps;

fn main() {
  let hkid_ops = HKIDOps::new();

  // Generate a random HKID with a known prefix
  let hkid = hkid_ops.generate_hkid(None, true).unwrap();
  println!("Random valid HKID: {}", hkid);

  // Generate with a specific prefix
  let hkid_a = hkid_ops.generate_hkid(Some("A"), true).unwrap();
  println!("HKID with prefix A: {}", hkid_a);

  // Generate with a custom (unknown) prefix
  let hkid_custom = hkid_ops.generate_hkid(Some("ZZ"), false).unwrap();
  println!("HKID with custom prefix: {}", hkid_custom);

  // Validate an HKID
  match hkid_ops.validate_hkid(&hkid_a, true) {
    Ok(valid) => println!("Is HKID valid? {}", valid),
    Err(e) => println!("Validation error: {}", e),
  }
}
```

Output:
```
Random valid HKID: T525548(6)
HKID with prefix A: A444227(2)
HKID with custom prefix: ZZ034129(A)
Is HKID valid? true
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
### Performance

The `generate_hkid` function was benchmarked on an Apple M1 Max (10-core CPU, 64 GB RAM) across four input scenarios:

- **Specific, known prefix:**  
  `generate_hkid(Some("A"), true)`  
  Generates an HKID using the provided prefix `"A"`, requiring that the prefix is recognized as a standard HKID prefix.

- **Specific, unchecked prefix:**  
  `generate_hkid(Some("A"), false)`  
  Generates an HKID using the provided prefix `"A"`, performing only format validation without requiring that the prefix appears in the standard prefix list.

- **Random, known prefix:**  
  `generate_hkid(None, true)`  
  Generates an HKID using a randomly selected standard HKID prefix from the HKID prefix enum.

- **Completely random prefix:**  
  `generate_hkid(None, false)`  
  Generates an HKID using a randomly generated prefix, which may or may not be a recognized standard prefix.

For each scenario, the function constructs the HKID prefix, generates six random digits, computes the check digit, and returns the formatted HKID string. Invalid or unrecognized prefixes are rejected based on the `must_exist_in_enum` parameter.

**Throughput:**  
All four scenarios measured approximately **2.2 million operations per second (OPS/sec)** on the specified hardware.

---
## License

This project is licensed under the [MIT License](./LICENSE).

You are free to use, modify, and distribute this software under the terms of the MIT License.
For detailed terms and conditions, please refer to the [LICENSE](./LICENSE) file in this repository.