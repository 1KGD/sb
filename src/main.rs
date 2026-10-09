#![feature(proc_macro_hygiene)]

use starbloom_derive::*;

#[sided(Client)]
mod client;
mod core;
mod player;

fn main() {
    crate::core::main(&[]);
}
