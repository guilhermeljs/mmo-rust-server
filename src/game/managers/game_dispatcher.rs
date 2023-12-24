use std::{sync::{mpsc::{Sender, Receiver}, Mutex, Arc}, thread, net::TcpStream};

use crate::{networking::packet_sender::{PacketSender, self}, game};

use super::game_world::GameWorld;

pub enum GameAction {
    SpawnPlayer(TcpStream, u32),
}

pub struct GameDispatcher {
    game_world: Arc<Mutex<GameWorld>>,
    channel_recv: Mutex<Receiver<GameAction>>,
    notify_sender: PacketSender
}

impl GameDispatcher {
    pub fn new(notify_sender: PacketSender, channel_recv: Mutex<Receiver<GameAction>>, world: Arc<Mutex<GameWorld>>) -> GameDispatcher{
        GameDispatcher {
            game_world: world,
            channel_recv: channel_recv,
            notify_sender: notify_sender
        }
    }

    pub fn init(mut self){
        thread::spawn(move || {
            loop {
                match self.channel_recv.lock().unwrap().recv(){
                    Ok(action)=>{
                        match action {
                            GameAction::SpawnPlayer(connection, player_id) => {
                                let player = self.game_world.as_ref().lock().expect("Error while locking thread.").spawn_player(player_id);
                                self.notify_sender.handle_action(packet_sender::NotifyAction::PlayerSpawned(Mutex::new(connection), &player, self.game_world.clone()));
                            }
                        }
                    },
                    Err(e)=>{
                        println!("Dispatcher mpsc channel error: {}", e);
                    }
                }
            }
        });
    }
}