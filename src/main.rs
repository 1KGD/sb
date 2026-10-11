use starbloom_core_shared::*;

#[sided(Client)]
mod client;
mod core;
mod player;

fn main() {
    crate::core::main(&[]);
}
