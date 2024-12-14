extern crate env_applier;

use env_applier::*;

fn main() -> () {
    let config = r#"{"test":"{{ HOME }}"}"#.apply();
    println!("My new config : {:?}", config);
}
