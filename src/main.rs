

use std::io::prelude::*;
use std::io;
use std::fs::File;

struct Board {
    cells: [i32; 81],
    holes: i32,
}
impl Board {
    fn new(board: Vec<i32>) -> Board {
        let mut b:[i32; 81] = [0; 81];
        let mut holes = 0;
        let mut c = 0;
        for i in board.iter() {
            b[c] = *i;
            if *i==0 { holes += 1; }
            c += 1;
        }
        Board {
            cells: b,
            holes: holes,
        } // todo: assert len==81
    }

    fn is_solved(&self) -> bool {
        self.holes == 0
    }

    fn remove(&mut self, pos: usize) {
        if self.cells[pos] != 0 {
            self.cells[pos] = 0;
            self.holes += 1;
        }
    }

    fn insert(&mut self, pos: usize, value: i32) -> bool {
        if value != 0 && self.legal(pos, value) {
           self.cells[pos] = value;
           self.holes -= 1;
           true
        } else { false }
    }

    fn legal(&self, pos: usize, value: i32) -> bool {
        for i in (0..9).map(|i| i*(pos/9)) { // row
            if i != pos && self.cells[i] == value {
                return false
            }
        }
        for i in (0..9).map(|i| i*9 + pos%9) { // column
            if i != pos && self.cells[i] == value {
                return false
            }
        }
        for i in self.get_box(pos) { // box
            if i != pos && self.cells[i] == value {
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

    fn print_board(&self) {
        let mut r = 1;
        for c in self.cells.iter() {
            if *c != 0 {
                if r%3==1 { print!("{}", c); }
                else { print!(" {}", c); }
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

struct Cell {
    value: i32, // 0 = blank, 1-9
}
impl Cell {
    fn new(value: i32) -> Cell { Cell { value: value }}
    fn get_value(&self) -> i32 { self.value }
}

fn play(mut board: Board) -> bool {
    println!("Insert:0 <index> <number> \nRemove:1 <0-80>");
    loop {
        board.print_board();

        let mut action = String::new();
        io::stdin().read_line(&mut action)
            .ok()
            .expect("failed to read line");

        let words: Vec<i32> = action
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if words[0] == 0 {
            println!("w");
            let _b = board.insert(words[1] as usize, words[2]);
        } else if words[0] == 1 {
            let _b = board.remove(words[1] as usize);
        } else {
            println!("Insert:0 <index> <number> \nRemove:1 <0-80>");
        }


        if board.is_solved() { return true };
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

    let mut board = Board::new(c);

    play(board);
}

fn main() {
    let c:[i32; 81] = [
        0,0,1,7,0,0,5,0,9,
        5,7,3,0,2,4,1,0,6,
        8,0,0,5,0,1,0,0,2,
        7,0,0,2,9,5,0,1,8,
        0,0,9,4,0,0,3,0,5,
        6,5,2,8,0,0,0,0,7,
        4,6,5,0,8,0,0,7,1,
        0,0,0,1,5,9,0,0,4,
        9,0,8,0,0,7,0,5,3];


    //let mut b = Board::new(c);
    shell();
    //b.print_board();
    //if !b.insert(0, 9) { println!("Not legal"); } else { println!("Legal move"); }
    //println!("Holes: {}", b.holes);
    //b.remove(3); b.remove(4);
    //b.print_board();

    //let v = [9;2];
    //for i in v.iter() {
    //    println!("{}", i);
    //}
}
