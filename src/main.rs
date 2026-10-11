use starbloom_core_shared::prelude::*;

#[sided(Client)]
mod client;
mod core;
mod player;
#[sided(Server)]
mod server;

fn main() {
    crate::core::main();
}
