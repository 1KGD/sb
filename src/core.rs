use std::ops::Deref;

use starbloom_core_shared::*;

const CORE_SHARED_PLUGINS: &'static [&dyn Plugin<SharedPlugin>] = &[];

const CORE_CLIENT_PLUGINS: &'static [&dyn Plugin<ClientPlugin>] = &[];

const CORE_SERVER_PLUGINS: &'static [&dyn Plugin<ServerPlugin>] = &[];

pub fn main() {
    let modded: bool = false;

    #[cfg(feature = "env_logger")]
    env_logger::builder()
        .format_timestamp(None)
        .filter(
            Some("starbloom"),
            if cfg!(feature = "debug_logging") {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .init();

    info!("STARBLOOM v{}", VERSION);
    debug!("IS_MOBILE = {}", *IS_MOBILE);

    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    for plugin in CORE_SHARED_PLUGINS {
        plugin.create(&mut world, &mut schedule);
    }

    for plugin in CORE_CLIENT_PLUGINS {
        plugin.create(&mut world, &mut schedule);
    }

    for plugin in CORE_SERVER_PLUGINS {
        plugin.create(&mut world, &mut schedule);
    }

    #[cfg(feature = "multiplayer")]
    {
        crate::client::main(world, schedule, modded);
    }
    #[cfg(not(feature = "multiplayer"))]
    crate::client::main(world, schedule, modded);
}
