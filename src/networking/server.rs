use std::{net::{TcpStream, TcpListener}, io::Read, thread, sync::{Arc, atomic::AtomicU32}};

use super::{netmsg::{NetMsg, self}, incoming_map::{self, IncomingPacket}, packet_handler::PacketHandler};

pub fn handle_packet_type(msg_buf: &mut NetMsg, con_id: u32, ph: Arc<PacketHandler>, stream: &mut TcpStream){
    while msg_buf.read_pos < msg_buf.msg_buffer.len() {
        match (msg_buf.read_u16()){
            Some(packet_size)=>{
                println!("Packet size: {}", packet_size);
                let buf = msg_buf.get_buffer(msg_buf.read_pos, usize::from(packet_size), true);
                let packet = IncomingPacket::from_netmsg(&mut NetMsg::new(buf.as_ref().to_vec()));
                ph.handle_packet(stream, con_id, packet)
            },
            None=>{
                break;
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, ph: Arc<PacketHandler>, ac: Arc<AtomicU32>){
    let mut buffer = [0; 1024];
    let con_id = ac.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    loop {
        // Lê a mensagem do cliente
        match stream.read(&mut buffer) {
            Ok(size) => {
                let mut msg_buf = NetMsg::new(buffer[0..size].to_vec());
                handle_packet_type(&mut msg_buf, con_id.clone(), Arc::clone(&ph), &mut stream);

                if size == 0{
                    break
                }
            }
            Err(e) => {
                eprintln!("Erro ao ler da stream: {}", e);
                break;
            }
        }
    }

    println!("SERVER: Conexão com cliente terminada.");
}

pub fn start_server(packet_handler: Arc<PacketHandler>, connection_counter: Arc<AtomicU32>) {
    thread::spawn(move || {
        println!("Servidor iniciado");
        let listener = TcpListener::bind("127.0.0.1:7171").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream)=>{
                    let handler_arc = Arc::clone(&packet_handler);
                    let counter_arc = Arc::clone(&connection_counter);

                    thread::spawn(move || handle_client(stream, handler_arc, counter_arc));
                }
                Err(err)=>{
                    eprintln!("Erro ao receber conexão: {}", err)
                }
            }
        }
    });
}