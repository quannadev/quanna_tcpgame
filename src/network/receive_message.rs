use crate::database::{MysqlDb, RedisDb};
use crate::managers::connections::ConnectionManager;
use amethyst::core::{bundle::SystemBundle, SystemDesc};
use amethyst::ecs::{DispatcherBuilder, Read, System, SystemData, World, Write};
use amethyst::network::simulation::{NetworkSimulationEvent, TransportResource};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::Result as AmethystResult;

pub struct SpamReceiveBundle {
    pub redis: RedisDb,
    pub mysql: MysqlDb,
}

impl<'a, 'b> SystemBundle<'a, 'b> for SpamReceiveBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> AmethystResult<()> {
        let manager = ConnectionManager::init(self.redis, self.mysql);
        world.insert(manager);
        builder.add(
            ReceiveSystemDesc::default().build(world),
            "receiving_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ReceiveSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, ReceiveSystem> for ReceiveSystemDesc {
    fn build(self, world: &mut World) -> ReceiveSystem {
        <ReceiveSystem as System<'_>>::SystemData::setup(world);
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
        let manager = world.try_fetch_mut::<ConnectionManager>().unwrap();
        ReceiveSystem::new(reader, manager.to_owned())
    }
}

pub struct ReceiveSystem {
    pub reader: ReaderId<NetworkSimulationEvent>,
    pub manager: ConnectionManager,
}

impl ReceiveSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>, manager: ConnectionManager) -> Self {
        Self { reader, manager }
    }
}

impl<'a> System<'a> for ReceiveSystem {
    type SystemData = (
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut net, channel): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    self.manager.on_message(*addr, payload, &mut net)
                }
                NetworkSimulationEvent::Connect(addr) => self.manager.on_connect(addr, &mut net),
                NetworkSimulationEvent::Disconnect(addr) => {
                    self.manager.on_disconnect(addr, &mut net)
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        }
    }
}
