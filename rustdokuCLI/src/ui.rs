use colored::*;
use scan_rules::scanner::Word;

use std::io;
//use std::collections::HashMap;

use logic::Board;
use logic::Cell;
use game::Move;
use game::MoveType;

pub enum MessageType {
    UnrecognizedCommand,
    Standard,
}

pub struct CLI {
    pub lines_printed: i32,
    pub message_type: MessageType,
    //pub message_map: HashMap<MessageType, String>,
}

impl CLI {
    pub fn new() -> CLI {
        //print!("\x1b[2J");  // Clear screen
        //print!("\x1b[H");  // Move cursor top left 
        CLI { 
            lines_printed: 0,
            message_type: MessageType::Standard,
            //message_map: HashMap<MessageType, String>::new(),
        }
    }

    pub fn render(&mut self, board: &Board) {
        self.reset_cursor_position();
        self.print_board(board);
        self.print_message();
    }

    fn reset_cursor_position(&mut self) {
        //print!("\x1b[H");  // Move cursor top left 
        //print!("\x1b[3J");  // Clear screen from cursor down
        while self.lines_printed > 0 {
            print!("\x1b[2K\x1b[A\r");
            self.lines_printed -= 1;
        }
    }

    fn print_message(&mut self) {
        match self.message_type {
            MessageType::Standard => print!("Please input your guess."),
            MessageType::UnrecognizedCommand => print!("Unrecognized command!"),
        }
        self.println();
    }

    pub fn parse_input(&mut self) -> Move {
        let mut move_ = Move {
            move_type: MoveType::NoOpt,
            i: 0, j: 0,
        };

        match self.parse_move() {
            Ok(mt) => move_ = mt,
            Err(_) => return move_,
        }

        move_
    }

    fn parse_move(&mut self) -> Result<Move, MessageType> {
        self.lines_printed += 1;
        self.message_type = MessageType::Standard;
        readln! {
            (let mt: Word<&str>, let i: i32, let j: i32) => {
                match mt {
                    "set" => return self.parse_pos(MoveType::Set),
                    //"unset" => return Err(MoveType::UnSet),
                    _ => (),
                }
            },
            (..other) => {}
        }
        Err(MessageType::UnrecognizedCommand)
    }

    fn parse_pos(&mut self, move_type: MoveType)
        -> Result<Move, MessageType> {
            Err(MessageType::UnrecognizedCommand)
    }

    fn print_board(&mut self, board: &Board) {
        let mut row_num = 0;
        for row in board.cells.iter() {
            for (col_num, cell) in row.iter().enumerate() {
                self.print_cell(&cell);

                if col_num == 2 || col_num == 5 {
                    print!(" | ");
                }
            }
            self.println();
            if row_num == 2 || row_num == 5 {
                print!(" -------------------------------");
                self.println();
            }
            row_num += 1;
        }
    }

    fn print_cell(&mut self, cell: &Cell) {
        if cell.value <= 0 {
            print!("   ");
        } else {
            if cell.read_only {
                print!(" {} ", cell.value.to_string().white());
            } else {
                print!(" {} ", cell.value.to_string().green());
            }
        }
    }

    fn println(&mut self) {
        println!();
        self.lines_printed += 1;
    }
}
