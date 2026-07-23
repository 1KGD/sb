use std::collections::vec_deque::VecDeque;

use egor::math::*;
use egor::render::*;

pub enum RenderCmd {
    Fill(Color),
    PositionedText(String, Vec2),
    Rect(Vec2, Vec2),
}

pub struct GfxCmds {
    buffer: VecDeque<RenderCmd>,
}

impl GfxCmds {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, cmd: RenderCmd) -> &mut Self {
        self.buffer.push_back(cmd);
        self
    }

    pub fn apply(&mut self, gfx: &mut Graphics<'_>) {
        while let Some(cmd) = self.buffer.pop_front() {
            match cmd {
                RenderCmd::Fill(color) => {
                    gfx.clear(color);
                }
                RenderCmd::PositionedText(text, pos) => {
                    gfx.text(&text).at(pos);
                }
                RenderCmd::Rect(pos, size) => {
                    gfx.rect().at(pos);
                }
            }
        }
    }
}
