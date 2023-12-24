use std::str::from_utf8;

#[derive(Clone)]
pub struct NetMsg {
    pub msg_buffer: Vec<u8>,
    pub read_pos: usize,
}


impl NetMsg {
    pub fn new(buffer: Vec<u8>) -> NetMsg{
        NetMsg {
            msg_buffer: buffer,
            read_pos: 0
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> &mut NetMsg{
        self.msg_buffer.extend_from_slice(bytes);
        self
    }

    pub fn write_byte(&mut self, byte: u8) -> &mut NetMsg{
        self.msg_buffer.push(byte);
        self
    }

    pub fn write_i16(&mut self, value: i16) -> &mut NetMsg{
        self.write_bytes(&value.to_be_bytes());
        self
    }

    pub fn write_u16(&mut self, value: u16) -> &mut NetMsg{
        self.write_bytes(&value.to_be_bytes());
        self
    }

    pub fn write_u32(&mut self, value: u32) -> &mut NetMsg{
        self.write_bytes(&value.to_be_bytes());
        self
    }

    pub fn add_packet_length(&mut self) -> &mut NetMsg {
        let length = self.msg_buffer.len();
        self.msg_buffer.splice(0..0, u16::to_be_bytes(length.try_into().expect("Error while converting usize to ushort")));
        self
    }

    pub fn write_i32(&mut self, value: i32) -> &mut NetMsg{
        self.write_bytes(&value.to_be_bytes());
        self
    }

    pub fn write_f32(&mut self, value: f32) -> &mut NetMsg{
        self.write_bytes(&value.to_be_bytes());
        self
    }

    pub fn write_string(&mut self, value: &str) -> &mut NetMsg{
        self.write_u16(value.len() as u16);
        self.write_bytes(value.as_bytes());
        self
    }

    fn can_read_buffer(&self, len: usize) -> bool{
        if self.read_pos + len >= self.msg_buffer.len() {
            return false
        }
        true
    }

    pub fn read_i16(&mut self) -> Option<i16>{
        if !self.can_read_buffer(2) {
            return None
        }
        let value = i16::from_be_bytes([self.msg_buffer[self.read_pos], self.msg_buffer[&self.read_pos+1]]);
        self.read_pos += 2;
        Some(value)
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if !self.can_read_buffer(2) {
            return None
        }

        let value = u16::from_be_bytes([self.msg_buffer[self.read_pos], self.msg_buffer[&self.read_pos+1]]);
        self.read_pos += 2;
        Some(value)
    }

    pub fn read_i32(&mut self) -> Option<i32>{
        if !self.can_read_buffer(4) {
            return None
        }

        let bytes: [u8; 4] = self.msg_buffer.as_slice()[self.read_pos..self.read_pos+4].try_into().expect("Error while reading");
        let value = i32::from_be_bytes(bytes);
        self.read_pos += 4;
        Some(value)
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if !self.can_read_buffer(4) {
            return None
        }

        let bytes: [u8; 4] = self.msg_buffer.as_slice()[self.read_pos..self.read_pos+4].try_into().expect("Error while reading");
        let value = u32::from_be_bytes(bytes);
        self.read_pos += 4;
        Some(value)
    }

    pub fn read_f32(&mut self) -> Option<f32>{
        if !self.can_read_buffer(4) {
            return None
        }

        let bytes: [u8; 4] = self.msg_buffer.as_slice()[self.read_pos..self.read_pos+4].try_into().expect("Error while reading");
        let value = f32::from_be_bytes(bytes);
        self.read_pos += 4;
        Some(value)
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if !self.can_read_buffer(1) {
            return None
        }

        let value: u8 = self.msg_buffer[self.read_pos];
        self.read_pos += 1;
        Some(value)
    }

    pub fn read_string(&mut self) -> Option<String>{
        let str_len = self.read_u16();
        match str_len {
            Some(str_len) => match from_utf8(&self.msg_buffer.as_slice()[self.read_pos.. self.read_pos + (usize::from(str_len))]) {
                Ok(s) => {
                    self.read_pos += usize::from(str_len);
                    Some(s.to_string())
                }
                Err(e) => {
                    println!("Error while reading string from netmsg.");
                    Some(String::new())
                }
            }
            None=> {
                None
            }
        }
    }

    pub fn get_buffer(&mut self, offset: usize, length: usize, increase_readpos: bool) -> Box<&[u8]>{
        let array: &[u8] = &self.msg_buffer.as_slice()[offset..(offset+length)];
        if increase_readpos {
            self.read_pos += length;
        }
        Box::new(array)
    }

    pub fn to_buffer(&self) -> Box<&[u8]>{
        let array: &[u8] = &self.msg_buffer.as_slice().clone();
        Box::new(array)
    }
}