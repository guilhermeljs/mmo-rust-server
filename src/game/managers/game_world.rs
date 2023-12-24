use crate::game::models::{objects::{Player, GameObject}, position::Position};

pub struct GameWorld {
    players: Vec<Player>
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            players: vec!()
        }
    }

    pub fn init(&mut self){
        
    }
    
    pub fn spawn_player(&mut self, id: u32) -> Player{
        println!("Spawnando jogador com id: {}", id);
        let player = Player::new(id, Position::new(2.0 * (id as f32),0.5,10.0), 300);
        self.players.push(player.clone());
        player
    }

    pub fn get_nearby_players(&mut self, pos: &Position) -> Vec<&Player> {
        let mut pl: Vec<&Player> = vec!();

        for player in &self.players {
            // TO DO: Refactor this part.
            if player.get_pos().distance(pos) < 10.0 {
                pl.push(player);
            }
        }

        pl
    }
}