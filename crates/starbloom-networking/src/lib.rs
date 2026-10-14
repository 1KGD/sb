use naia_client::*;
use naia_shared::*;
use starbloom_core_shared::prelude::*;

fn protocol() -> Protocol {
    Protocol::builder()
        .tick_interval(std::time::Duration::from_millis(40))
        .build()
}

pub struct NetworkingPlugin;

impl Plugin<SharedPlugin> for NetworkingPlugin {
    fn create(&self, world: &mut World, schedule: &mut Schedule) {
        world.insert_non_send(NetworkingManager::new());
    }
}

pub struct NetworkingManager {
    client: Client<Entity>,
}

impl NetworkingManager {
    pub fn new() -> Self {
        let mut client: Client<Entity> = Client::<Entity>::new(ClientConfig::default(), protocol());
        let socket = naia_client::transport::webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());

        client.connect(socket);

        Self { client }
    }
}
