use bevy_ecs::prelude::*;
use const_format::concatcp;
use egor::math::*;

mod assets;
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
