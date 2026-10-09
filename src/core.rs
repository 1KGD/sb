use starbloom_base::prelude::*;
use starbloom_derive::*;
use starbloom_map::*;
use starbloom_tiles::*;
use starbloom_worldgen::*;

const CORE_PLUGINS: &'static [&dyn Plugin] = &[
    &MapPlugin,
    &DefaultTilePlugin,
    &WorldgenPlugin,
];

pub fn main(mod_plugins: &'static [&dyn Plugin]) {
    let modded: bool = !mod_plugins.is_empty();

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

    for plugin in CORE_PLUGINS {
        plugin.create(&mut world, &mut schedule);
    }

    for plugin in mod_plugins {
        plugin.create(&mut world, &mut schedule);
    }

    #[cfg(feature = "multiplayer")]
    {
        #[sided(Client)]
        crate::client::main(world, schedule, modded);
    }
    #[cfg(not(feature = "multiplayer"))]
    crate::client::main(world, schedule, modded);
}
