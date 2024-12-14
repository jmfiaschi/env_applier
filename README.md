# env_applier
[![Actions Status](https://github.com/jmfiaschi/env_applier/workflows/CI/badge.svg)](https://github.com/jmfiaschi/env_applier/actions)

Apply environment variables on a string.

# Getting Started
## Quick Start
```rust
extern crate env_applier;

use env_applier::*;

fn main() -> () {
    let config = r#"{"test":"{{ HOME }}"}"#.to_string().apply();
    println!("My new config : {:?}", config);
}
```

or with prefix

```rust
extern crate env_applier;

use env_applier::*;

fn main() -> () {
    let config = r#"{"test":"{{ HOME }}"}"#.to_string().apply_with_prefix("MY_PREFIX");
    println!("My new config : {:?}", config);
}
```

## Bench
```
time:   [607.07 ns 613.29 ns 624.77 ns]
```