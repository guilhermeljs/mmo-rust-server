use super::netmsg::NetMsg;

pub enum IncomingPacket {
    LoginPacket { login: String, password: String },
    DisconnectionPacket,
    UnknownPacket(u16)
}

impl IncomingPacket {
    pub fn from_netmsg(msg: &mut NetMsg) -> Self {
        msg.read_pos = 0;

        let id = msg.read_u16();
        match id {
            Some(id)=>{
                match id{
                    1 => {
                        let login = msg.read_string().expect("Malformatted login packet");
                        let pass = msg.read_string().expect("Malformatted login packet");
                        
                        IncomingPacket::LoginPacket{ login: login, password: pass}
                    },
                    _ => {
                        Self::UnknownPacket(id)
                    }
                }
            },
            None=>Self::DisconnectionPacket
        }
    }
}
