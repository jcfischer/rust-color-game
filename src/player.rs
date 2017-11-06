//! Player logic

use graphics::types::Color;

/// Stores player information
#[derive(Copy, Clone)]
pub struct Player {

    /// stores the players color
    pub color: Color,
}

impl Player {
    /// creates a new player
    pub fn new(color: Color) -> Player {
        Player {
            color: color,
        }
    }
}
