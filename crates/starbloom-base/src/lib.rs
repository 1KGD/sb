use bevy_ecs::prelude::*;
use const_format::concatcp;
use egor::math::*;
use log::*;

mod bootstrap;
pub mod prelude;
mod render;

pub fn is_mobile_user_agent() -> bool {
    let user_agent = web_sys::window().and_then(|win| win.navigator().user_agent().ok());

    match user_agent {
        Some(ua) => {
            let ua_lower = ua.to_lowercase();
            ua_lower.contains("mobi")
                || ua_lower.contains("android")
                || ua_lower.contains("iphone")
                || ua_lower.contains("ipad")
        }
        None => false,
    }
}

pub const VERSION: &'static str = concatcp!(
    env!("CARGO_PKG_VERSION"),
    if cfg!(debug_assertions) { "+DEV" } else { "" }
);

pub static IS_MOBILE: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
    cfg!(target_os = "android")
        || if cfg!(target_arch = "wasm32") {
            is_mobile_user_agent()
        } else {
            false
        }
});

pub trait Plugin {
    fn create(world: &mut World, schedule: &mut Schedule);
}

#[derive(Default, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn from_vec2(&mut self, vec: Vec2) {
        self.x = vec.x;
        self.y = vec.y;
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(GameStateManager::new());
        schedule.configure_sets(GameState::Bootstrap.run_if(
            |state_manager: Res<GameStateManager>| state_manager.state == GameState::Bootstrap,
        ));
        schedule.configure_sets(GameState::Mainloop.run_if(
            |state_manager: Res<GameStateManager>| state_manager.state == GameState::Mainloop,
        ));
        schedule.add_systems(on_state_switch);
    }
}

fn on_state_switch(state_manager: Res<GameStateManager>) {
    if state_manager.is_changed() {
        info!(
            "Switched game state to {:#?} within this frame",
            state_manager.state
        );
    }
}

#[derive(Resource)]
pub struct GameStateManager {
    state: GameState,
}

impl GameStateManager {
    fn new() -> Self {
        Self {
            state: GameState::Bootstrap,
        }
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Bootstrap,
    Mainloop,
}
