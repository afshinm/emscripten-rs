#![feature(link_args)]
#[link_args = "-s BUILD_AS_WORKER=1"]

use std::str::FromStr;

extern crate libc;

mod externs;

extern {}

fn main() {}

#[no_mangle]
pub extern fn worker_fn(a: &str) {
    println!("Boooo");
    externs::worker_respond_provisionally("", 0);
    externs::worker_respond("", 0);
}
