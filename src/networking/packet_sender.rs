use std::{collections::HashMap, net::TcpStream, io::Write, sync::{Mutex, Arc}};

use crate::{game::{managers::game_world::GameWorld, models::objects::{Player, GameObject}}, networking::{outgoing_map::OutgoingPacket, packet_sender}};

pub enum NotifyAction<'a> {
    PlayerSpawned(Mutex<TcpStream>, &'a Player, Arc<Mutex<GameWorld>>)
}

pub struct PacketSender {
    connections: HashMap<u32, Mutex<TcpStream>>
}

impl PacketSender {
    pub fn new() -> Self {
        PacketSender { 
            connections: HashMap::new()
        }
    }

    fn get_player_con(&self, p: &Player) -> Option<&Mutex<TcpStream>>{
        match self.connections.get(&p.get_id()) {
            Some(con)=>{
                if !PacketSender::is_stream_alive(&mut con.lock().expect("Couldn't lock in get_player_con")) {
                    return None
                }
                Some(con)
            },
            None=>None
        }
    }

    fn is_stream_alive(stream: &mut TcpStream) -> bool {
        let result = stream.write(OutgoingPacket::to_netmsg(OutgoingPacket::PingClient).to_buffer().as_ref());
        result.is_ok()
    }

    pub fn handle_action(&mut self, action: NotifyAction){
        match action {
            NotifyAction::PlayerSpawned(con, player, world)=>{
                let packet = OutgoingPacket::EntitySpawned { entity_id: player.get_id(), position: player.get_pos().clone() };
                let msg = OutgoingPacket::to_netmsg(packet);
                let buf = msg.to_buffer();
                
                for pl in world.lock().expect("Error while locking world mutex").get_nearby_players(player.get_pos()){
                    match self.get_player_con(pl) {
                        Some(conn)=>{
                            conn.lock().expect("Couldn't lock TcpStream").write(buf.as_ref()).unwrap_or_default();

                            let client_packet = OutgoingPacket::EntitySpawned { entity_id: pl.get_id(), position: pl.get_pos().clone() };
                            let to_msg = OutgoingPacket::to_netmsg(client_packet);
                            let to_buf = to_msg.to_buffer();
                            let _ = &con.lock().expect("Couldn't lock TcpStream").write(to_buf.as_ref()).unwrap_or_default();
                        },
                        None=>{
                            let client_packet = OutgoingPacket::EntitySpawned { entity_id: pl.get_id(), position: pl.get_pos().clone() };
                            let to_msg = OutgoingPacket::to_netmsg(client_packet);
                            let to_buf = to_msg.to_buffer();
                            let _ = &con.lock().expect("Couldn't lock TcpStream").write(to_buf.as_ref()).unwrap_or_default();
                        }
                    }
                }

                let _ = &con.lock().expect("Couldn't lock TcpStream").write(buf.as_ref()).unwrap_or_default();

                self.connections.insert(player.get_id().clone(), con);
            }
        }
    }
}