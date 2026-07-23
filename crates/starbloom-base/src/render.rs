use egor::app::*;
use egor::render::*;

pub enum RenderCmd {
    Fill(Color),
}

pub struct GfxCmds {
    buffer: Vec<RenderCmd>,
}

impl GfxCmds {
    pub fn new() -> Self {
        Self { buffer: vec![] }
    }

    pub fn apply(&mut self, gfx: &mut Graphics<'_>) {
        while let Some(cmd) = self.buffer.pop() {
            match cmd {
                RenderCmd::Fill(color) => {
                    gfx.clear(color);
                }
            }
        }
    }
}
