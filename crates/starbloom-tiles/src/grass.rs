use crate::*;

pub(crate) fn regester_grass(regestry: &mut Mut<TileRegestry>) {
    regestry.regester("starbloom:air", Tile::declare(None));
    regestry.regester(
        "starbloom:grass",
        Tile::declare(Some(include_bytes!("../assets/grass_clean.png"))),
    );
    regestry.regester(
        "starbloom:grass_rough",
        Tile::declare(Some(include_bytes!("../assets/grass_rough.png"))),
    );
}
