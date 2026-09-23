use crate::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Air;

#[derive(Component, Debug, Clone, Copy)]
pub struct Grass;

#[derive(Component, Debug, Clone, Copy)]
pub struct GrassRough;

pub(crate) fn regester_grass(world: &mut World) {
    let air: Tile = Tile::declare(world, None, Air);
    world
        .get_resource_mut::<TileRegestry>()
        .unwrap()
        .regester(air);
    let grass: Tile = Tile::declare(
        world,
        Some(include_bytes!("../assets/grass_clean.png")),
        Grass,
    );
    world
        .get_resource_mut::<TileRegestry>()
        .unwrap()
        .regester(grass);
    let grass_rough: Tile = Tile::declare(
        world,
        Some(include_bytes!("../assets/grass_rough.png")),
        GrassRough,
    );
    world
        .get_resource_mut::<TileRegestry>()
        .unwrap()
        .regester(grass_rough);
}
