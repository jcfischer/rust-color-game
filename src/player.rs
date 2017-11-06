//! Player logic

use graphics::types::Color;

use brain::Brain;

/// Stores player information
#[derive(Copy, Clone)]
pub struct Player {

    /// stores the players color
    pub color: Color,
    ///
    pub brain: Brain,
}

impl Player {
    /// creates a new player
    pub fn new(color: Color, brain: Brain) -> Player {
        Player {
            color: color,
            brain: brain,
        }
    }
}
