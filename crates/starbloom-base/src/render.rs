use egor::app::*;
use log::*;

pub struct Renderer<'a>(pub *mut &'a mut FrameContext<'a>);

impl<'a> Renderer<'a> {
    pub fn new() -> Self {
        Self(std::ptr::null_mut())
    }

    pub fn ctx(&mut self) -> Option<&mut &'a mut FrameContext<'a>> {
        if self.0.is_null() {
            error!("Expired FrameContext");
            return None;
        }
        unsafe { self.0.as_mut() }
    }
}
