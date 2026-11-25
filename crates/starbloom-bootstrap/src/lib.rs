use bevy_ecs::prelude::*;
use egor::render::*;
use log::*;
use starbloom_base::*;
use starbloom_renderer::*;
use starbloom_states::*;

pub struct BootstrapPlugin;

impl Plugin<ClientPlugin> for BootstrapPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        world.insert_resource(BootstrapMananger::new());
        schedule.add_systems(
            (load_assets, finish_bootstrap)
                .chain()
                .in_set(ClientState::Bootstrap),
        );
    }
}

pub struct TextureRegestry {
    regestry: Vec<Texture>,
}

impl TextureRegestry {
    fn new() -> Self {
        Self {
            regestry: Vec::new(),
        }
    }

    fn regester_texture(&mut self, texture: Texture) -> usize {
        self.regestry.push(texture);
        self.regestry.len() - 1
    }

    fn resolve_texture(&self, id: usize) -> Option<&Texture> {
        self.regestry.get(id)
    }
}

pub fn load_assets(
    mut commands: Commands,
    mut manager: ResMut<BootstrapMananger>,
    mut texture_regestry: NonSendMut<TextureRegestry>,
    query: Query<(Entity, &AssetRequest)>,
    mut renderer: NonSendMut<Renderer>,
) {
    if manager.phase == BootstrapPhase::Assets {
        for (entity, asset) in &query {
            match asset {
                AssetRequest::Texture(texture_data) => {
                    let texture: Texture = renderer.load_texture(texture_data);
                    let id: usize = texture_regestry.regester_texture(texture);
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

fn finish_bootstrap(mut state_manager: ResMut<ClientStateManager>) {
    state_manager.switch(if cfg!(feature = "intro") {
        ClientState::Intro
    } else {
        ClientState::MainMenu
    });
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

#[derive(Component, Clone, Copy, Debug)]
pub struct TextureAsset {
    id: usize,
}

impl TextureAsset {
    pub fn resolve<'a>(
        &self,
        texture_regestry: &'a NonSendMut<'a, TextureRegestry>,
    ) -> Option<&'a Texture> {
        texture_regestry.resolve_texture(self.id)
    }
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
