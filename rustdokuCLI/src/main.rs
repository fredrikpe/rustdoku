extern crate colored;
#[macro_use] extern crate scan_rules;

mod board_import;
mod game;
mod logic;
mod ui;


fn main() {
    let mut game = game::Game::new();

    let mut ui = ui::CLI::new();

    // Game loop
    loop {
        ui.render(&game.board);

        let move_ = ui.parse_input();

        game.update(move_);
    }
}
