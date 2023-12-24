use std::net::TcpStream;
use std::io::{Read, Write};

use crate::networking::netmsg::NetMsg;

pub fn split_buffer(mut netmsg: NetMsg, id: &u8){
    match netmsg.read_u16() {
        Some(length)=>{
            println!("f. rdpos {}, len {}, bytes left {}", netmsg.read_pos, netmsg.msg_buffer.len(), netmsg.msg_buffer.len() - netmsg.read_pos); 
            let mut msg = NetMsg::new(netmsg.get_buffer(netmsg.read_pos,usize::from(length), true).as_ref().to_vec());
            handle_packet(msg, id);

            if netmsg.read_pos < netmsg.msg_buffer.len() {

                split_buffer(netmsg, id);
            }
        },
        None => {

        }
    }
}

pub fn handle_packet(mut msg: NetMsg, id: &u8){
    match msg.read_u16() {
        Some(v)=>{
            match v {
                1 => { println!("CLIENT {}: Received player spawn packet. Len: {}-----------------------------------{}", id, msg.msg_buffer.len(), id)}
                _ => { println!("CLIENT {}: Unknwon packet received", id)}
            }
        }
        None=>{

        }
    }
}

pub fn connect(id: u8) {
    match TcpStream::connect("localhost:7171") {
        Ok(mut stream) => {
            println!("CLIENT: Successfully connected to server in port 7171");

            let mut msg = NetMsg::new(vec!());
            msg.write_u16(1);
            msg.write_string("teste");
            msg.write_string("1234");

            stream.write(&msg.to_buffer().as_mut()).unwrap();
            println!("CLIENT: Sent login packet, awaiting reply...");
            
            let mut players_spawned: Vec<u32> = vec!();
            let counter = 0;

            let mut data = [0 as u8; 1024]; // using 6 byte buffer
            loop {
                match stream.read(&mut data) {
                    Ok(read) => {
                        let mut received_msg = NetMsg::new(data[0..read].to_vec());
                        split_buffer(received_msg, &id);
                    },
                    Err(e) => {
                        println!("CLIENT: Failed to receive data: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("CLIENT: Failed to connect: {}", e);
        }
    }

    println!("CLIENT: Terminated.");
}