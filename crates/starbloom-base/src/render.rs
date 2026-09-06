use std::collections::vec_deque::VecDeque;

use bevy_ecs::prelude::*;
use egor::math::*;
use egor::render::*;

type RenderCmd = Box<dyn FnOnce(&mut Graphics<'_>)>;

pub struct GfxCmds {
    buffer: VecDeque<RenderCmd>,
}

impl GfxCmds {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    pub fn draw(&mut self, cmd: RenderCmd) -> &mut Self {
        self.buffer.push_back(cmd);
        self
    }

    pub fn apply(&mut self, gfx: &mut Graphics<'_>) {
        while let Some(cmd) = self.buffer.pop_front() {
            cmd(gfx);
        }
    }
}
