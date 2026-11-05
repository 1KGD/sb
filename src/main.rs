use starbloom_core_shared::prelude::*;

#[sided(Client)]
mod client;
mod core;
mod player;
#[cfg(feature="remote_host")]
mod server;

fn main() {
    crate::core::main();
}
