

use ::logic::board::*;

pub struct Step {
    num: i32,
    pos: usize,
}

impl Step {
    fn new(num: i32, pos: usize) -> Step {
        Step {
            num: num,
            pos: pos,
        }
    }
}

// Only one possible location
pub fn unique_candidate(b: &Board) -> Option<Step> {
    // for each row ,c, b
    for i in b.get_representative() {
        let s = match unique(b.get_row(i), b) {
            Some(s) => Some(s),
            _ => match unique(b.get_col(i), b) {
                Some(s) => Some(s),
                _ => match unique(b.get_col(i), b) {
                    Some(s) => Some(s),
                    None => None,
                }            
            }
        }; 
        match s {
            Some(v) => return Some(v),
            _ => (),
        };
    }
    None
}

fn unique(area: Vec<usize>, b: &Board) -> Option<Step> {
    let mut c = [0; 9];
    let mut pos = [100; 9];
    for i in area {
       for v in 0..9 {
           if b.cells[i].number == 0 && b.cells[i].marks[v+1] == Mark::Legal {
               c[v] += 1;
               pos[v] = i;
           }
       }
    }
    for (i, n) in c.iter().enumerate() {
        assert!(*n != 0);
        if *n == 1 {
            assert!(pos[i] != 100);
            return Some(Step::new(*n, pos[i]))
        }
    }
    None
}

/*
// Removing candidates
pub fn line_remove(b: Board) -> {
}
pub fn block_remove(b: Board) -> {
}
pub fn color_remove(b: Board) {
}


// 2, 3, ...
pub fn naked_subset(b: Board) {
}

// Redundant. hidden implies the existence of a naked.
//pub fn hidden_subset(b: Board) {
//}

pub fn x_wing(b: Board) {
}

pub fn color_decide(b: Board) {
}

pub fn xy_wing(b: Board) {
}

pub fn swordfish(b: Board) {
}*/
