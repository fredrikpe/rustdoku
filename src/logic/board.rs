

//use std::io::prelude::*;
//use std::io;


#[derive(Clone, Copy, PartialEq)]
pub enum Mark {
    Legal,
    Illegal,
    Unmarked,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub marks: [Mark; 10],
    pub number: i32,
    pub read_only: bool,
}

impl Cell {
    pub fn new(ro: bool) -> Cell {
        Cell {
            marks: [Mark::Illegal; 10],
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
    pub fn new(board_vec: Vec<i32>) -> Board {
        let mut cells = [Cell::new(true); 81];
        let mut holes = 0;
        for (i, n) in board_vec.iter().enumerate() {
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

    pub fn solve_singles(&mut self) {
        for i in 0..81 {
            if self.cells[i as usize].number == 0 {
                let mut c = 0;
                let mut k = 0;
                for (j, m) in self.cells[i as usize].marks.iter().enumerate() {
                    if *m == Mark::Legal {
                        c += 1;
                        k = j;
                    }
                }
                if c == 1 {
                    self.insert(i as usize, k as i32);
                }
            }
        }
    }

    pub fn fill_marks(&mut self) {
        for i in 0..81 {
            if self.cells[i as usize].number == 0 {
                for j in 1..10 {
                    if self.cells[i as usize].marks[j as usize] != Mark::Unmarked {
                        if self.legal(i as usize, j) {
                            self.cells[i as usize].marks[j as usize] = Mark::Legal;
                        } else {
                            self.cells[i as usize].marks[j as usize] = Mark::Illegal;
                        }
                    }
                }
            }
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

    pub fn mark(&mut self, pos: usize, m: usize) {
        if self.cells[pos].number == 0 && self.legal(pos, m as i32) {
            if self.cells[pos].marks[m] == Mark::Legal {
                self.cells[pos].marks[m] = Mark::Unmarked;
            } else {
                self.cells[pos].marks[m] = Mark::Legal;
            }
        }
    }

    pub fn remove(&mut self, pos: usize) {
        if !self.cells[pos].read_only && self.cells[pos].number != 0 {
            self.cells[pos].number = 0;
            self.holes += 1;
        }
    }

    pub fn insert(&mut self, pos: usize, value: i32) {
        if !self.cells[pos].read_only && value != 0 && self.legal(pos, value) {
           self.cells[pos].number = value;
           self.cells[pos].marks = [Mark::Illegal; 10];
           self.holes -= 1;
        }
    }

    pub fn legal(&self, pos: usize, value: i32) -> bool {
        for i in self.get_row(pos)  { // row
            if i != pos && self.cells[i].number == value {
                return false
            }
        }
        for i in self.get_col(pos) { // column
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

    pub fn get_row(&self, pos: usize) -> Vec<usize> {
        (0..9).map(|i| i + 9*(pos/9)).collect::<Vec<usize>>()
    }

    pub fn get_col(&self, pos: usize) -> Vec<usize> {
        (0..9).map(|i| i*9 + pos%9).collect::<Vec<usize>>()
    }

    pub fn get_box(&self, pos: usize) -> Vec<usize> {
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

    pub fn get_representative(&self) -> Vec<usize> {
        vec!(0, 13, 24, 28, 42, 52, 56, 66, 79) 
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
