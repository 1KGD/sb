use starbloom_core_shared::prelude::*;

#[cfg(not(feature = "client"))]
pub(crate) fn main(world: World, schedule: Schedule, modded: bool) {
    info!("Server started!");
}
