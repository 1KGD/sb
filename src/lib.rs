#![feature(proc_macro_hygiene)]

use starbloom_derive::*;

#[sided(Client)]
mod client;
mod core;
mod player;

#[cfg(feature = "client")]
egor::main!(main);
#[allow(dead_code)]
fn main() {
    crate::core::main(&[]);
}
