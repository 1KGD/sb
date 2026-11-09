use bevy_ecs::prelude::*;
use egor::app::*;
use egor::math::*;
use egor::render::*;

pub const MIN_DIMENSION: IVec2 = ivec2(256, 256);

#[derive(Resource)]
pub struct FrameData {
    pub screen_size: Vec2,
    pub delta: f32,
    pub frame: u64,
}

impl From<&FrameContext<'_>> for FrameData {
    fn from(ctx: &FrameContext<'_>) -> Self {
        Self {
            screen_size: MIN_DIMENSION.as_vec2(),
            delta: ctx.timer.delta.clamp(0., 1. / 16.), // No massive jumps
            frame: ctx.timer.frame,
        }
    }
}

pub struct Renderer<'a>(pub *mut Graphics<'a>);

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl<'a> std::ops::Deref for Renderer<'a> {
    type Target = Graphics<'a>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref_unchecked() }
    }
}

impl<'a> std::ops::DerefMut for Renderer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut_unchecked() }
    }
}
