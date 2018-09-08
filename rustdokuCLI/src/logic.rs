
#[derive(Copy, Clone)]
pub struct Board {
    pub cells: [[Cell; 9]; 9],
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub value: i32,
    pub read_only: bool,
}

impl Board {
    pub fn new(board_vec: Vec<i32>) -> Board {
        let mut b = Board {
            cells: [[Cell {
                value: 0,
                read_only: false,
            }; 9]; 9],
        };

        for (i, n) in board_vec.iter().enumerate() {
            b.cells[i / 9][i % 9].value = *n;
            if *n > 0 {
                b.cells[i / 9][i % 9].read_only = true;
            }
        }
        b
    }

    pub fn set_value(&mut self, (i, j): (i32, i32), value: i32) -> bool {
        if !self.cell(i, j).read_only
        && value != 0 && self.legal((i, j), value) {
            self.cell(i, j).value = value;
            return true
        }
        false
    }

    pub fn unset_value(&mut self, (i, j): (i32, i32)) -> bool {
        if !self.cell(i, j).read_only {
            self.cell(i, j).value = 0;
            return true
        }
        false
    }

    pub fn legal(&mut self, (i, j): (i32, i32), value: i32) -> bool {
        // Row & Column
        for k in 0..9 {
            if i != k && self.cell(k, j).value == value {
                return false;
            }
            if j == k && self.cell(i, k).value == value {
                return false;
            }
        }
        // Box
        //for (k, l) in self.get_box((i, j)) {
            //if (i, j) == (k, l) && self.cells[k][l].value == value {
                //return false;
            //}
        //}
        true
    }

    /*
    pub fn get_box(&self, (i, j): (i32, i32)) -> Vec<(i32, i32)> {
        let start_i = i / 3 * 3;
        let start_j = i / 3 * 3;

        let a = (start_i..start_i + 3);
        let b = (start_j..start_j + 3);

        a.iter().zip(b.iter());
    }*/

    fn cell(&mut self, i: i32, j: i32) -> &mut Cell {
        &mut self.cells[i as usize][j as usize]
    }
}
