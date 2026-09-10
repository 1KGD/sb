use egor::app::*;
use log::*;

pub struct Renderer<'a>(pub *mut &'a mut &'a mut FrameContext<'a>);

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }

    pub fn ctx(&mut self) -> Option<Box<&'a mut &'a mut FrameContext<'a>>> {
        if self.0.is_null() {
            error!("Expired FrameContext");
            return None;
        }
        Some(unsafe { Box::from_raw(self.0) })
    }
}
