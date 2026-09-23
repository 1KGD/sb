use bevy_ecs::prelude::*;
use log::*;

use crate::render::*;
use crate::*;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn create(world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(BootstrapMananger::new());
        schedule.add_systems(load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    mut manager: ResMut<BootstrapMananger>,
    query: Query<(Entity, &AssetRequest)>,
    mut renderer: NonSendMut<Renderer>,
) {
    if manager.phase == BootstrapPhase::Assets {
        if let Some(ctx) = renderer.ctx() {
            for (entity, asset) in &query {
                match asset {
                    AssetRequest::Texture(texture_data) => {
                        let id: usize = ctx.gfx.load_texture(texture_data);
                        debug!(
                            "Loaded texture with entity id {} and texture id {}",
                            entity, id
                        );
                        commands
                            .entity(entity)
                            .insert(TextureAsset { id })
                            .remove::<AssetRequest>();
                    }
                }
            }
            debug!("Done loading assets");
            manager.phase = BootstrapPhase::Handoff;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BootstrapPhase {
    Assets,
    Handoff,
}

#[derive(Resource)]
pub struct BootstrapMananger {
    pub phase: BootstrapPhase,
}

impl BootstrapMananger {
    pub fn new() -> Self {
        Self {
            phase: BootstrapPhase::Assets,
        }
    }
}

#[derive(Component)]
pub enum AssetRequest {
    Texture(&'static [u8]),
}

#[derive(Component)]
pub struct TextureAsset {
    pub id: usize,
}

pub fn declare_texture_asset(
    world: &mut World,
    texture_data: &'static [u8],
    bundle: impl Bundle + std::fmt::Debug + Copy,
) -> Entity {
    let entity: Entity = world
        .spawn((bundle, AssetRequest::Texture(texture_data)))
        .id();
    debug!("Spawned texture asset request with bundle {:#?}", bundle);
    entity
}
