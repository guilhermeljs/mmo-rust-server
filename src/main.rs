mod echo_client;
mod networking; 
mod game;

use std::{sync::{Arc, atomic::AtomicU32, Mutex, mpsc}, thread};

use game::managers::{game_world::GameWorld, game_dispatcher::{GameDispatcher, GameAction}};
use networking::{packet_handler::PacketHandler, packet_sender::PacketSender};

fn main() {
    // Primeiro, inicializar jogo
    let mut game_world = GameWorld::new();
    

    //Inicializar dispatcher (atua como uma ponte entre o jogo e a networking)
    let notify_sender = PacketSender::new();
    let (sender, receiver) = mpsc::channel::<GameAction>();
    let game_dispatcher = GameDispatcher::new(notify_sender, Mutex::new(receiver), Arc::new(Mutex::new(game_world)));
    game_dispatcher.init();

    // Por último inicializar a networking
    let packet_handler = PacketHandler::new(Arc::new(Mutex::new(sender.clone())));

    let connection_counter = AtomicU32::new(1);
    networking::server::start_server(Arc::new(packet_handler), Arc::new(connection_counter));

    /*for i in 0..5 {
        thread::spawn(move || {
            echo_client::connect(i);
        });
    }*/
    //echo_client::connect();

    loop {

    }
}
