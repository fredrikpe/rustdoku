use colored::*;

use std::io;

use logic::Board;
use logic::Cell;
use game::Move;
use game::MoveType;

pub enum MessageType {
    UnrecognizedMove,
}

pub struct CLI {
    pub lines_printed: i32,
    pub message_type: MessageType,
}

impl CLI {
    pub fn new() -> CLI {
        print!("\x1b[2J");  // Clear screen
        print!("\x1b[H");  // Move cursor top left 
        CLI { lines_printed: 0 }
    }

    pub fn render(&mut self, board: &Board) {
        self.reset_cursor_position();
        self.print_board(board);
        self.print_message();
    }

    fn reset_cursor_position(&mut self) {
        print!("\x1b[H");  // Move cursor top left 
        print!("\x1b[3J");  // Clear screen from cursor down
        //while self.lines_printed > 0 {
            //print!("\x1b[2K\x1b[A\r");
            //self.lines_printed -= 1;
        //}
    }

    fn print_message(&mut self) {
        print!("Please input your guess.");
        self.println();
    }

    pub fn parse_input(&mut self) -> Move {
        let mut input = String::new();


        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        self.lines_printed += 1;

        let mut words = input.split_whitespace();

        let mt = self.parse_move(words.nth(0));

        //let match words[1].parse::<i32>() {
            //Ok(v) => 
            //Err(e) => 


        Move {
            move_type: mt,
            i: 2,
            j: 1,
        }
    }

    fn parse_move(&mut self, input: Option<&str>) -> MoveType {
        match input {
            Some("s") => MoveType::Set,
            Some("u") => MoveType::Set,
            _ => {
                self.message_type = MessageType::UnrecognizedCommand;
                MoveType::NoOpt
            }
        }
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
