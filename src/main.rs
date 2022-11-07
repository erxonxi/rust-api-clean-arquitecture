mod internal;
mod rest;

use std::env;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }

    if let Err(e) = rest::build() {
        println!("ERROR: {:?}!", e);
    }
}
