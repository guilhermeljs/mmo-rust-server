use std::{sync::{mpsc::Sender, Arc, Mutex}, net::TcpStream};

use crate::game::managers::game_dispatcher::GameAction;

use super::incoming_map::IncomingPacket;

pub struct PacketHandler {
    dispatcher: Arc<Mutex<Sender<GameAction>>>
}

impl PacketHandler {
    pub fn new(dispatcher: Arc<Mutex<Sender<GameAction>>>) -> PacketHandler {
        PacketHandler {
            dispatcher: dispatcher,
        }
    }

    pub fn handle_packet(&self, stream: &mut TcpStream, con_id: u32, packet: IncomingPacket) {
        match packet {
            IncomingPacket::LoginPacket {login, password} => {
                println!("Received login packet: {} {}", login, password);
                // TO DO: Database
                if login == "teste" && password == "1234" {
                    println!("Personagem logou com sucesso!");
                    let _ = self.dispatcher.as_ref().lock().unwrap().send(GameAction::SpawnPlayer(stream.try_clone().unwrap(), con_id));
                }
            },
            IncomingPacket::UnknownPacket(id) => {
                println!("Unknown packet received: {}", id)
            },
            IncomingPacket::DisconnectionPacket => {
                stream.shutdown(std::net::Shutdown::Both).expect("Erro ao desligar stream");
            }
        }
    }
}