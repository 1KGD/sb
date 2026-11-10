#[cfg(not(feature = "mobile_build"))]
pub mod entrypoint {
    use starbloom_core_shared::prelude::*;
    #[sided(Client)]
    pub mod client;
    pub mod core;
    pub mod player;
    #[cfg(feature = "remote_host")]
    pub mod server;
}

#[cfg(not(feature = "mobile_build"))]
fn main() {
    crate::entrypoint::core::main();
}
