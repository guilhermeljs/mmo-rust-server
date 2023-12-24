use super::position::Position;

macro_rules! implement_game_object {
    ($struct_name:ident) => {
        impl GameObject for $struct_name {
            fn get_pos(&self) -> &Position {
                &self.position
            }
        
            fn get_sprite_id(&self) -> u32{
                self.sprite_id
            }
        
            fn get_id(&self) -> u32 {
                self.id
            }
        }
    };
}

pub trait GameObject {
    fn get_pos(&self) -> &Position;
    fn get_sprite_id(&self) -> u32;
    fn get_id(&self) -> u32;
}

#[derive(Clone)]
pub struct Player {
    id: u32,
    position: Position,
    sprite_id: u32
}
implement_game_object!(Player);

impl Player {
    pub fn new(id: u32, pos: Position, sprite_id: u32) -> Self {
        Player {
            id: id,
            sprite_id: sprite_id,
            position: pos
        }
    }
}