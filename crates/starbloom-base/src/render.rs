use bevy_ecs::prelude::*;
use egor::app::*;
use egor::math::*;
use log::*;

#[derive(Resource)]
pub struct FrameData {
    pub screen_size: Vec2,
    pub delta: f32,
    pub frame: u64,
}

impl From<&FrameContext<'_>> for FrameData {
    fn from(ctx: &FrameContext<'_>) -> Self {
        Self {
            screen_size: ctx.gfx.screen_size(),
            delta: ctx.timer.delta,
            frame: ctx.timer.frame,
        }
    }
}

pub struct Renderer<'a>(pub *mut FrameContext<'a>);

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }

    pub fn ctx(&mut self) -> Option<&mut FrameContext<'a>> {
        if self.0.is_null() {
            error!("Expired FrameContext");
            return None;
        }
        unsafe { self.0.as_mut() }
    }
}
