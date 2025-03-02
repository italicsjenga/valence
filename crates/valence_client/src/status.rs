use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_core::protocol::{packet_id, Decode, Encode, Packet};

use crate::event_loop::{EventLoopSchedule, EventLoopSet, PacketEvent};

pub(super) fn build(app: &mut App) {
    app.add_event::<RequestRespawnEvent>()
        .add_event::<RequestStatsEvent>()
        .add_system(
            handle_status
                .in_schedule(EventLoopSchedule)
                .in_base_set(EventLoopSet::PreUpdate),
        );
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct RequestRespawnEvent {
    pub client: Entity,
}

pub struct RequestStatsEvent {
    pub client: Entity,
}

fn handle_status(
    mut packets: EventReader<PacketEvent>,
    mut respawn_events: EventWriter<RequestRespawnEvent>,
    mut request_stats_events: EventWriter<RequestStatsEvent>,
) {
    for packet in packets.iter() {
        if let Some(pkt) = packet.decode::<ClientStatusC2s>() {
            match pkt {
                ClientStatusC2s::PerformRespawn => respawn_events.send(RequestRespawnEvent {
                    client: packet.client,
                }),
                ClientStatusC2s::RequestStats => request_stats_events.send(RequestStatsEvent {
                    client: packet.client,
                }),
            }
        }
    }
}

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(id = packet_id::CLIENT_STATUS_C2S)]
pub enum ClientStatusC2s {
    PerformRespawn,
    RequestStats,
}
