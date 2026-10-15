use naia_server::*;
use naia_shared::*;
use starbloom_core_shared::prelude::*;
use starbloom_networking::*;

#[cfg(not(feature = "client"))]
pub(crate) fn main(mut world: World, mut schedule: Schedule, _modded: bool) {
    info!("Starting dedicated server...");

    let mut networked_world: NetworkedWorld<'_> = NetworkedWorld::new(&mut world);

    let mut server: Server<Entity> = Server::new(ServerConfig::default(), protocol());
    let socket = transport::webrtc::Socket::new(
        &transport::webrtc::ServerAddrs::default(),
        server.socket_config(),
    );
    server.listen(socket);

    info!("Dedicated server started!");
    loop {
        server.receive_all_packets();
        server.process_all_packets(&mut networked_world, &Instant::now());

        server.take_world_events();

        for _ in server.take_tick_events(&Instant::now()).read::<TickEvent>() {
            schedule.run(&mut *networked_world);
        }

        server.send_all_packets(&mut networked_world);

        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
