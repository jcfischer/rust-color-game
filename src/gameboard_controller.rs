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
    pub current_index: usize,
}

impl GameboardController {
    /// Creates a new gameboard controller
    pub fn new(gameboard: Gameboard, players: Vec<Player>) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
            players: players,
            current_index: 0,
        }
    }

    fn current_player(&self) -> Player {
        return self.players[self.current_index];
    }
    fn next_player(&mut self) {
        self.current_index += 1;
        self.current_index %= self.players.len();
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
                let cell_x = (x / size * 8.0) as usize;
                let cell_y = (y / size * 8.0) as usize;
                let player = self.current_player();
                self.gameboard.set_player([cell_x, cell_y], player);
            }
            
            self.next_player();
            
            let player = self.current_player();
            if !player.brain.is_human {
                let pos = self.gameboard.random_free_position();
                let cell_x = pos[0];
                let cell_y = pos[1];
                self.gameboard.set_player([cell_x, cell_y], player);
                self.next_player();
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
