use std::ops::{Deref, DerefMut};

use bevy_ecs::prelude::*;
use const_format::concatcp;
use egor::math::*;

pub mod prelude;
#[cfg(feature = "client")]
mod render;

#[cfg(feature = "web")]
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
    if cfg!(debug_assertions) { " DEV" } else { "" }
);

#[allow(unreachable_code)]
pub static IS_MOBILE: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
    #[cfg(target_os = "android")]
    return true;
    #[cfg(feature = "web")]
    return is_mobile_user_agent();
    false
});

pub trait Plugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule);
}

#[derive(Default, Component)]
pub struct Position(Vec2);

impl Deref for Position {
    type Target = Vec2;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Vec2> for Position {
    #[inline]
    fn as_ref(&self) -> &Vec2 {
        self.deref()
    }
}

impl From<Position> for Vec2 {
    #[inline]
    fn from(value: Position) -> Self {
        *value
    }
}

impl From<Vec2> for Position {
    #[inline]
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}
