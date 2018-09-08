
use logic::Board;
use board_import;

pub enum MoveType {
    Set,
    UnSet,
    Undo,
    NoOpt,
}

pub struct Move {
    pub move_type: MoveType,
    pub i: i32,
    pub j: i32,
}

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: board_import::board_from_file(1),
        }
    }

    pub fn update(&mut self, move_: Move) {
        match move_.move_type {
            MoveType::Set => self.board.set_value((move_.i, move_.j), 78),
            MoveType::UnSet => self.board.unset_value((move_.i, move_.j)),
            _ => false,
        };
    }
}

