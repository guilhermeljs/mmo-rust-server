use crate::game::models::position::Position;

use super::netmsg::NetMsg;

pub enum OutgoingPacket {
    PingClient,
    EntitySpawned{ entity_id: u32, position: Position}
}

impl OutgoingPacket {
    pub fn to_netmsg(packet: OutgoingPacket) -> NetMsg{
        let mut msg = NetMsg::new(vec!());

        match packet {
            OutgoingPacket::PingClient => {
                msg.write_u16(0);
                msg.add_packet_length();
                msg
            },
            OutgoingPacket::EntitySpawned{entity_id, position} => {
                msg.write_u16(1);
                msg.write_u32(entity_id);
                msg.write_f32(position.x);
                msg.write_f32(position.y);
                msg.write_f32(position.z);
                msg.add_packet_length();
                msg
            },
        }
    }
}