#[cfg(feature = "enabled")]
pub mod prelude {
    pub use egor::app::*;
    pub use egor::render::*;
    pub use starbloom_bootstrap::*;
    pub use starbloom_camera::*;
    pub use starbloom_input::*;
    pub use starbloom_renderer::*;
}
#[cfg(not(feature = "enabled"))]
pub mod prelude {}
