

use std::io::prelude::*;
use std::io;
use std::fs::File;


#[derive(Clone, Copy)]
pub struct Cell {
    pub marks: [bool; 10],
    pub number: i32,
    pub read_only: bool,
}

impl Cell {
    pub fn new(ro: bool) -> Cell {
        Cell {
            marks: [false; 10],
            number: 0,
            read_only: ro,
        }
    }
}


pub struct Board {
    pub cells: [Cell; 81],
    holes: i32,
}

impl Board {
    pub fn new() -> Board {
        let mut f = File::open("sudokus/1.txt").ok().expect("failed fileopen");

        let mut s = String::new();
        f.read_to_string(&mut s).ok().expect("failed read_to_string");

        let c: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
               
        let mut cells = [Cell::new(true); 81];
        let mut holes = 0;
        for (i, n) in c.iter().enumerate() {
            cells[i].number = *n;
            if *n == 0 { 
               cells[i].read_only = false;
               holes += 1;
            }
        }
        Board {
            cells: cells,
            holes: holes,
        }
    }

    pub fn fill_board(&mut self, board: Vec<i32>) {
        self.holes = 0;
        for (i, n) in board.iter().enumerate() {
            self.cells[i].number = *n;
            if *n == 0 { 
               self.cells[i].read_only = false;
               self.holes += 1;
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        self.holes == 0
    }

    pub fn remove(&mut self, pos: usize) {
        if !self.cells[pos].read_only && self.cells[pos].number != 0 {
            self.cells[pos].number = 0;
            self.holes += 1;
        }
    }

    pub fn mark(&mut self, pos: usize, m: usize) {
        if self.cells[pos].number == 0 {
            self.cells[pos].marks[m] = !self.cells[pos].marks[m];
        }
    }

    pub fn insert(&mut self, pos: usize, value: i32) {
        if !self.cells[pos].read_only && value != 0 && self.legal(pos, value) {
           self.cells[pos].number = value;
           self.cells[pos].marks = [false; 10];
           self.holes -= 1;
        }
    }

    pub fn legal(&self, pos: usize, value: i32) -> bool {
        for i in (0..9).map(|i| i + 9*(pos/9)) { // row
            if i != pos && self.cells[i].number == value {
                return false
            }
        }
        for i in (0..9).map(|i| i*9 + pos%9) { // column
            if i != pos && self.cells[i].number == value {
                return false
            }
        }
        for i in self.get_box(pos) { // box
            if i != pos && self.cells[i].number == value {
                return false
            }
        }
        true
    }

    fn get_box(&self, pos: usize) -> Vec<usize> {
        let x = match pos % 9 {
            0|1|2 => 0,
            3|4|5 => 1,
            _ => 2,
        };
        let y = match pos / 9 {
            0|1|2 => 0,
            3|4|5 => 1,
            _ => 2,
        };
        let mut v = (0..3).map(|n| n+(3*x)+9*(3*y)).collect::<Vec<usize>>();
        v.extend((0..3).map(|n| n+(3*x)+9*(3*y+1)));
        v.extend((0..3).map(|n| n+(3*x)+9*(3*y+2)));
        v
    }

    pub fn print_board(&self) {
        let mut r = 1;
        for c in self.cells.iter() {
            if c.number != 0 {
                if r%3==1 { print!("{}", c.number); }
                else { print!(" {}", c.number); }
            } else {
                if r%3==1 { print!(" "); }
                else { print!("  "); }
            }
            if r%9==0 { println!(""); }
            if r%3==0 && r%9 != 0 { print!("|"); }
            if (r/9)%3==0 && (r/9)%9!=0 && r%9==0 {
                print!("-----------------\n");
            }
            r += 1;
        }
        println!("");
    }
}


fn play(mut board: Board) -> bool {
    println!("Insert:0 <index> <number> \nRemove:1 <0-80>");
    board.print_board();
    loop {


        let mut action = String::new();
        io::stdin().read_line(&mut action)
            .ok()
            .expect("failed to read line");

        let words:Vec<&str> = action.split_whitespace().collect();

        if words[0] == "0" {
            board.insert(words[1].parse::<usize>().unwrap(), words[2].parse::<i32>().unwrap());
            if board.cells[words[1].parse::<usize>().unwrap()].number ==  words[2].parse::<i32>().unwrap() {
                println!("Insertion successful!");
                board.print_board();
            } else {
                println!("Illegal insertion!")
            }
        } else if words[0] == "1" {
            board.remove(words[1].parse::<usize>().unwrap());
            println!("Removal successful!");
            board.print_board();
        } else {
            println!("Insert:0 <index> <number> \nRemove:1 <0-80>");
        }


        if board.is_solved() {
            println!("Congratulations!");
            return true
        };
    }
}

fn shell() {
    println!("Hi and welcome to rustdoku!");
    println!("Please choose sudoku game (1-9)");

    let mut game = String::new();

    io::stdin().read_line(&mut game)
        .ok()
        .expect("failed to read line");

    let game: u32 = game.trim().parse()
        .ok()
        .expect("Please type a number!");
    let mut f = match game {
        1 => File::open("sudokus/1.txt").ok().expect("failed fileopen"),
        _ => File::open("sudokus/1.txt").ok().expect("failed fileopen"),
    };

    let mut s = String::new();
    f.read_to_string(&mut s).ok().expect("failed read_to_string");

    let c: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut board = Board::new();
    board.fill_board(c);

    play(board);
}
