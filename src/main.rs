use starbloom_core_shared::prelude::*;

#[sided(Client)]
mod client;
mod core;
mod player;

fn main() {
    crate::core::main();
}
