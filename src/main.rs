extern crate env_applier;

use env_applier::*;
use std::env::*;

fn main() -> () {
    let mut config = r#"{"test":"{{ HOME }}"}"#.to_string();
    config = Vars::apply(config);

    println!("My new config : {:?}", config);
}
