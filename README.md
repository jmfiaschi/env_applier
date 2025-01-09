# env_applier
[![Actions Status](https://github.com/jmfiaschi/env_applier/workflows/ci/badge.svg)](https://github.com/jmfiaschi/env_applier/actions/workflows/ci.yml)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg)](https://github.com/semantic-release/semantic-release)
![crates.io](https://img.shields.io/crates/v/env_applier.svg)

env_applier is a Rust crate that applies environment variables to placeholders in a string, making configuration handling simple and flexible.

# Getting Started

## Installation

Add the following line to your Cargo.toml:
```toml
[dependencies]
env_applier = "x.y.z" # Replace with the latest version
```

## Quick Start

### Basic Example
```rust
extern crate env_applier;

use env_applier::*;

fn main() -> () {
    let config = r#"{"test":"{{ HOME }}"}"#.to_string().apply();
    println!("My new config : {:?}", config);
}
```

In this example, {{ HOME }} is replaced with the value of the HOME environment variable.

---

### Example with Prefix
```rust
extern crate env_applier;

use env_applier::*;

fn main() -> () {
    let config = r#"{"test":"{{ HOME }}"}"#.to_string().apply_with_prefix("MY_PREFIX");
    println!("My new config : {:?}", config);
}
```
When using `apply_with_prefix`, only environment variables prefixed with `MY_PREFIX` will be considered.

---

## Useful link

* [Benchmark report](https://jmfiaschi.github.io/env_applier/bench/main/)
* [Package](https://crates.io/crates/env_applier)

---

## Contributing

Contributions are welcome!

To contribute:

1. Fork the repository and create your branch (git checkout -b feature/my-feature).
2. Commit your changes (git commit -m 'Add some feature').
3. Push to the branch (git push origin feature/my-feature).
4. Open a pull request.

For major changes, please open an issue first to discuss your proposal.

Please ensure that tests are added or updated as appropriate.

---

## License

Licensed under either of the following, at your option:

* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT License](https://choosealicense.com/licenses/mit/)
