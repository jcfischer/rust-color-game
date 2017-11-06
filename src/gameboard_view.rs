//! Gameboard view

use graphics::types::Color;
use graphics::{Context, Graphics};

use GameboardController;

/// Stores gameboard view settings
pub struct GameboardViewSettings {
    /// position from top left corner.
    pub position: [f64; 2],
    /// Siez of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color
    pub background_color: Color,
    /// Border color
    pub border_color: Color,
    /// Edge color around the whole board
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections
    pub section_edge_color: Color,
    /// Edge color between cells
    pub cell_edge_color: Color,
    /// Edge radius around the whole board
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections
    pub section_edge_radius: f64,
    /// Edge radius between cells
    pub cell_edge_radius: f64,
    /// Selected cell background color.
    pub selected_cell_background_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
        }
    }
}

/// Stores visual information about a gameboard
pub struct GameboardView {
    /// Stores gameboard view settings
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }

    /// Draw gameboard
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
        use graphics::Rectangle;

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

        // Draw board background
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        // Draw selected cell background.
        if let Some(ind) = controller.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
            let cell_rect = [
                settings.position[0] + pos[0], settings.position[1] + pos[1],
                cell_size, cell_size
            ];
            Rectangle::new(settings.selected_cell_background_color)
                .draw(cell_rect, &c.draw_state, c.transform, g);
        }

        // // Draw Cell borders
        // let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        // for i in 0..9 {

        //     let x = settings.position[0] + i as f64 / 9.0 * settings.size;
        //     let y = settings.position[1] + i as f64 / 9.0 * settings.size;

        //     let x2 = settings.position[0] + settings.size;
        //     let y2 = settings.position[1] + settings.size;

        //     let vline = [x, settings.position[1], x, y2];
        //     cell_edge.draw(vline, &c.draw_state, c.transform, g);

        //     let hline = [settings.position[0], y, x2, y];
        //     cell_edge.draw(hline, &c.draw_state, c.transform, g);
        // }

        for j in 0..7 {
            for i in 0..7 {
                if let Some(color) = controller.gameboard.color([i, j]) {
                    let cell_size = settings.size / 9.0;
                    let pos = [i as f64 * cell_size, j as f64 * cell_size];
                    let cell_rect = [
                        settings.position[0] + pos[0], settings.position[1] + pos[1],
                        cell_size, cell_size
                    ];
                    Rectangle::new(color)
                        .draw(cell_rect, &c.draw_state, c.transform, g);
                }
            }
        }

      // Draw board edge.
      Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);
    }
}
