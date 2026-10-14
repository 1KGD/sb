#[cfg(feature = "client")]
use starbloom_core_client::prelude::*;
use starbloom_core_shared::prelude::*;

use crate::player::*;

const CORE_SHARED_PLUGINS: &'static [&dyn Plugin<SharedPlugin>] = &[
    &PlayerPlugin,
    #[cfg(feature = "multiplayer")]
    &starbloom_networking::NetworkingPlugin,
];

#[cfg(feature = "client")]
const CORE_CLIENT_PLUGINS: &'static [&dyn Plugin<ClientPlugin>] = &[
    &GameStatePlugin,
    &BootstrapPlugin,
    &FrameStepPlugin,
    &CameraPlugin,
    &InputPlugin,
];

#[cfg(feature = "server")]
const CORE_SERVER_PLUGINS: &'static [&dyn Plugin<ServerPlugin>] = &[];

#[allow(unreachable_code)]
pub fn main() {
    let modded: bool = false;

    #[cfg(feature = "web")]
    {
        wasm_logger::init(
            wasm_logger::Config::new(if cfg!(feature = "debug_logging") {
                Level::Debug
            } else {
                Level::Info
            })
            .module_prefix("starbloom"),
        );
        console_error_panic_hook::set_once();
    }

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

    let mut world: World = World::new();
    let mut schedule: Schedule = Schedule::default();

    for plugin in CORE_SHARED_PLUGINS {
        plugin.create(&mut world, &mut schedule);
    }

    #[cfg(feature = "client")]
    for plugin in CORE_CLIENT_PLUGINS {
        plugin.create(&mut world, &mut schedule);
    }

    #[cfg(feature = "server")]
    for plugin in CORE_SERVER_PLUGINS {
        plugin.create(&mut world, &mut schedule);
    }

    #[cfg(not(feature = "client"))]
    crate::server::main(world, schedule, modded);

    #[cfg(feature = "client")]
    crate::client::main(world, schedule, modded);
}
