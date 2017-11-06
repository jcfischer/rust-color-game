//! Gameboard controller

use piston::input::GenericEvent;

use Gameboard;
use Player;

/// Handles events for the Sudoku game
pub struct GameboardController {
    /// stores the gameboard state
    pub gameboard: Gameboard,
    /// Selected cell
    pub selected_cell: Option<[usize; 2]>,
    /// Stores last cursor position
    pub cursor_pos: [f64; 2],
    /// Stores the players
    pub players: Vec<Player>,
    /// store the current player
    pub player: Player,
}

impl GameboardController {
    /// Creates a new gameboard controller
    pub fn new(gameboard: Gameboard, players: Vec<Player>) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
            players: players,
            player: players[0],
        }
    }


    /// Handles Events
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }



        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper corner
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // check that coordinates are inside boundary
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // compute the cell position
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
                self.gameboard.set_player([cell_x, cell_y], &self.player);
            }
        }

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn set_cell() {
    }

}
