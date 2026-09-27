use egor::app::*;
use log::*;

pub struct Renderer<'a>(pub *mut &'a mut [&'a mut FrameContext<'a>; 1]);

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }

    pub fn ctx(&mut self) -> Option<&mut &'a mut FrameContext<'a>> {
        if self.0.is_null() {
            error!("Expired FrameContext");
            return None;
        }
        unsafe { Box::from_raw(self.0).get_mut(0) }
    }
}
