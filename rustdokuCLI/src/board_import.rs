use std::fs::File;
use std::io::Read;

use logic::Board;

pub fn board_from_file(num: i32) -> Board {
    let filename = format!("sudokus/{}.txt", num);
    let mut f = File::open(filename).ok().expect("failed fileopen");
    let mut s = String::new();
    f.read_to_string(&mut s)
        .ok()
        .expect("failed read_to_string");

    let vec = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    Board::new(vec)
}
