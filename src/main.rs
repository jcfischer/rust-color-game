#![deny(missing_docs)]


//! A Sudoku game

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::RenderEvent;

pub use brain::Brain;
pub use player::Player;
pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

use gameboard::{COLOR_1, COLOR_2, COLOR_3};

mod brain;
mod player;
mod gameboard;
mod gameboard_controller;
mod gameboard_view;


fn main() {
    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new("Tic-Tac-Go", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();

    let player_1 = Player::new(COLOR_1, Brain::new(true));
    let player_2 = Player::new(COLOR_2, Brain::new(true));
    let player_3 = Player::new(COLOR_3, Brain::new(false));

    let players = vec![player_1, player_2, player_3];

    let mut gameboard_controller = GameboardController::new(gameboard, players);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(gameboard_view.settings.position,
                                   gameboard_view.settings.size,
                                   &e);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, &c, g);
            });
        }
    }


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

}
