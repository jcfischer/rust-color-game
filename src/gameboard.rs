//! Game board logic
extern crate rand;

use graphics::types::Color;

use Brain;
use Player;

/// Size of game board
const SIZE: usize = 8;

pub const COLOR_0 : Color = [1.0, 1.0, 1.0, 1.0];
pub const COLOR_1 : Color = [0.0, 0.32, 0.28, 1.0];
pub const COLOR_2 : Color = [0.68, 0.19, 0.04, 1.0];
pub const COLOR_3 : Color = [0.24, 0.69, 0.47, 1.0];

/// Stores game board information
#[derive(Copy, Clone)]
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell
    pub cells: [[Player; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new game board
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[Player::new(COLOR_0, Brain::new(true)); SIZE]; SIZE],
        }
    }


     /// Gets the color at cell location.
    pub fn player(&self, ind: [usize; 2]) -> Player {
        return self.cells[ind[0]][ind[1]];
    }

    /// Set cell value to a player
    pub fn set_player(&mut self, ind: [usize; 2], player: Player) {
        self.cells[ind[0]][ind[1]] = player;
    }
    
    ///
    pub fn random_free_position(&mut self) -> [usize; 2] {
        loop {
            let x = rand::random::<usize>() % SIZE;
            let y = rand::random::<usize>() % SIZE;
            
            let c = self.cells[x][y].color;
            println!("{}:{} - {}-{}-{}-{}", x, y, c[0], c[1], c[2], c[3]);
            
            if self.cells[x][y].color == COLOR_0 {
                return [x, y];
            }
        }
    }
}
