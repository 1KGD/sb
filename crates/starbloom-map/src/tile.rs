pub struct Tile<'a> {
    pub behavior: &'a (dyn TileBehavior + Sync),
}

pub trait TileBehavior {
    fn invisible(&self) -> bool;
}
