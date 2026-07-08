pub struct Tile {
    pub renderable: bool
}

impl Tile {
    pub const fn declare(renderable: bool) -> Self {
        Self {
            renderable
        }
    }
}
