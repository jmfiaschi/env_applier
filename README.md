# env_applier
[![Actions Status](https://github.com/jmfiaschi/env_applier/workflows/CI/badge.svg)](https://github.com/jmfiaschi/env_applier/actions)
Apply environment variables on an object.

# Getting Started
## Quick Start
`
extern crate env_applier;

use env_applier::*;
use std::env::*;

fn main() -> () {
    let mut config = r#"{"test":"{{ HOME }}"}"#.to_string();
    config = Vars::apply(config);

    println!("My new config : {:?}", config);
}
`
## Bench
`
    time:   [233.55 us 233.94 us 234.35 us]
    change: [-4.4260% -2.9642% -1.5991%] (p = 0.00 < 0.05)
`