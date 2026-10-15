use std::ops::{Deref, DerefMut};

#[sided(Client)]
use naia_client::*;
use naia_shared::*;
use starbloom_core_shared::prelude::*;

pub fn protocol() -> Protocol {
    Protocol::builder()
        .tick_interval(std::time::Duration::from_millis(40))
        .build()
}

#[sided(Client)]
pub struct ClientNetworkingPlugin;

#[sided(Client)]
impl Plugin<ClientPlugin> for ClientNetworkingPlugin {
    fn create(&self, world: &mut World, _schedule: &mut Schedule) {
        world.insert_non_send(ClientNetworkingManager::new());
    }
}

#[sided(Client)]
pub struct ClientNetworkingManager {
    client: Client<Entity>,
}

#[sided(Client)]
impl ClientNetworkingManager {
    pub fn new() -> Self {
        let mut client: Client<Entity> = Client::<Entity>::new(ClientConfig::default(), protocol());
        let socket = naia_client::transport::webrtc::Socket::new(
            "http://127.0.0.1:14191",
            client.socket_config(),
        );

        client.connect(socket);

        Self { client }
    }
}

#[sided(Server)]
pub struct ServerNetworkingPlugin;

#[sided(Server)]
impl Plugin<ServerPlugin> for ServerNetworkingPlugin {
    fn create(&self, _world: &mut World, _schedule: &mut Schedule) {}
}

pub struct NetworkedWorld<'w> {
    world: &'w mut World,
}

impl<'w> Deref for NetworkedWorld<'w> {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        self.world
    }
}

impl<'w> DerefMut for NetworkedWorld<'w> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.world
    }
}

impl<'w> NetworkedWorld<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self { world }
    }
}

impl<'w> WorldRefType<Entity> for &mut NetworkedWorld<'w> {
    fn has_entity(&self, _world_entity: &Entity) -> bool {
        todo!()
    }

    fn entities(&self) -> Vec<Entity> {
        todo!()
    }

    fn has_component<R: ReplicatedComponent>(&self, _world_entity: &Entity) -> bool {
        todo!()
    }

    fn has_component_of_kind(&self, _world_entity: &Entity, _component_kind: &ComponentKind) -> bool {
        todo!()
    }

    fn component<'a, R: ReplicatedComponent>(
        &'a self,
        _entity: &Entity,
    ) -> Option<ReplicaRefWrapper<'a, R>> {
        todo!()
    }

    fn component_of_kind<'a>(
        &'a self,
        _entity: &Entity,
        _component_kind: &ComponentKind,
    ) -> Option<ReplicaDynRefWrapper<'a>> {
        todo!()
    }
}

impl<'w> WorldMutType<Entity> for &mut NetworkedWorld<'w> {
    fn spawn_entity(&mut self) -> Entity {
        self.world.spawn_empty().id()
    }

    fn local_duplicate_entity(&mut self, _world_entity: &Entity) -> Entity {
        todo!()
    }

    fn local_duplicate_components(&mut self, _mutable_entity: &Entity, _immutable_entity: &Entity) {
        todo!()
    }

    fn despawn_entity(&mut self, world_entity: &Entity) {
        self.world.entity_mut(*world_entity).despawn();
    }

    fn component_kinds(&mut self, _world_entity: &Entity) -> Vec<ComponentKind> {
        todo!()
    }

    fn component_mut<'a, R: ReplicatedComponent>(
        &'a mut self,
        _world_entity: &Entity,
    ) -> Option<ReplicaMutWrapper<'a, R>> {
        todo!()
    }

    fn component_mut_of_kind<'a>(
        &'a mut self,
        _world_entity: &Entity,
        _component_kind: &ComponentKind,
    ) -> Option<ReplicaDynMutWrapper<'a>> {
        todo!()
    }

    fn component_apply_update(
        &mut self,
        _converter: &dyn LocalEntityAndGlobalEntityConverter,
        _world_entity: &Entity,
        _component_kind: &ComponentKind,
        _update: ComponentUpdate,
    ) -> std::prelude::v1::Result<(), SerdeErr> {
        todo!()
    }

    fn component_apply_field_update(
        &mut self,
        _converter: &dyn LocalEntityAndGlobalEntityConverter,
        _world_entity: &Entity,
        _component_kind: &ComponentKind,
        _update: ComponentFieldUpdate,
    ) -> std::prelude::v1::Result<(), SerdeErr> {
        todo!()
    }

    fn mirror_entities(&mut self, mutable_entity: &Entity, immutable_entity: &Entity) {
        self.world
            .entity_mut(*immutable_entity)
            .clone_components::<()>(*mutable_entity);
    }

    fn mirror_components(
        &mut self,
        _mutable_entity: &Entity,
        _immutable_entity: &Entity,
        _component_kind: &ComponentKind,
    ) {
        todo!()
    }

    fn insert_component<R: ReplicatedComponent>(
        &mut self,
        _world_entity: &Entity,
        _component_ref: R,
    ) {
        todo!()
    }

    fn insert_boxed_component(
        &mut self,
        _world_entity: &Entity,
        _boxed_component: Box<dyn Replicate>,
    ) {
        todo!()
    }

    fn remove_component<R: ReplicatedComponent>(&mut self, _world_entity: &Entity) -> Option<R> {
        todo!()
    }

    fn remove_component_of_kind(
        &mut self,
        _world_entity: &Entity,
        _component_kind: &ComponentKind,
    ) -> Option<Box<dyn Replicate>> {
        todo!()
    }

    fn entity_publish(
        &mut self,
        _component_kinds: &ComponentKinds,
        _converter: &dyn EntityAndGlobalEntityConverter<Entity>,
        _global_world_manager: &dyn GlobalWorldManagerType,
        _world_entity: &Entity,
    ) {
        todo!()
    }

    fn component_publish(
        &mut self,
        _component_kinds: &ComponentKinds,
        _converter: &dyn EntityAndGlobalEntityConverter<Entity>,
        _global_world_manager: &dyn GlobalWorldManagerType,
        _world_entity: &Entity,
        _component_kind: &ComponentKind,
    ) {
        todo!()
    }

    fn entity_unpublish(&mut self, _world_entity: &Entity) {
        todo!()
    }

    fn component_unpublish(&mut self, _world_entity: &Entity, _component_kind: &ComponentKind) {
        todo!()
    }

    fn entity_enable_delegation(
        &mut self,
        _component_kinds: &ComponentKinds,
        _converter: &dyn EntityAndGlobalEntityConverter<Entity>,
        _global_world_manager: &dyn GlobalWorldManagerType,
        _world_entity: &Entity,
    ) {
        todo!()
    }

    fn component_enable_delegation(
        &mut self,
        _component_kinds: &ComponentKinds,
        _converter: &dyn EntityAndGlobalEntityConverter<Entity>,
        _global_world_manager: &dyn GlobalWorldManagerType,
        _world_entity: &Entity,
        _component_kind: &ComponentKind,
    ) {
        todo!()
    }

    fn entity_disable_delegation(&mut self, _world_entity: &Entity) {
        todo!()
    }

    fn component_disable_delegation(
        &mut self,
        _world_entity: &Entity,
        _component_kind: &ComponentKind,
    ) {
        todo!()
    }
}
