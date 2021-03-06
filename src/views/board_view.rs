
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;


use ::phi::{Phi, View, ViewAction};
use ::sdl2::rect::{Point, Rect};
//use ::std::path::Path;

use ::logic::{Board, Cell};
use ::logic::board::Mark;

use ::views::settings;

pub struct MenuView;

#[derive(PartialEq)]
enum Helper {
    Off     = 0,
    Marks   = 1,
    On      = 2,
}

enum Move {
    IMove(usize, i32),
    MMove(usize, i32),
}

enum InsertMode {
    Normal,
    Mark,
    Highlight,
}

pub struct BoardView {
    board: Board,
    focus: usize,
    highlight: i32,
    square_colores: [::sdl2::pixels::Color; 81],
    helper: Helper,
    insert_mode: InsertMode,
    moves: Vec<Move>,
    fonts: settings::Fonts,
}


impl BoardView {
    pub fn new(phi: &mut Phi, board: i32) -> BoardView {
        BoardView {
            board: Board::new(board_from_file(board)),
            focus: 0,
            highlight: -1,
            square_colores: [settings::WHITE; 81],
            insert_mode: InsertMode::Mark,
            helper: Helper::On,
            moves: vec!(),
            fonts: settings::Fonts::new(),
        }
    }

    fn move_focus(&mut self, i: i32) {
        match i {
            1 => if self.focus < 80 { self.focus += 1 },
            -1 => if self.focus > 0 { self.focus -= 1 },
            9 => if self.focus < 72 { self.focus += 9 },
            -9 => if self.focus > 8 { self.focus -= 9 },
            _ => panic!("Wrong focus move")
        };
    }

    fn undo(&mut self) {
        // Bugged when inserting the removed mark. f ex. remove mark 3, insert 3
        // , undo then mark 3 is there
        match self.moves.pop() {
            Some(Move::MMove(pos, i)) => self.board.mark(pos, i as usize),
            Some(Move::IMove(pos, i)) => self.board.remove(pos),
            None => (),
        };
    }

    fn num_press(&mut self, num: i32) {
        match self.insert_mode {
            InsertMode::Mark => {
                self.moves.push(Move::MMove(self.focus, num));
                self.board.mark(self.focus, num as usize)
                },
            InsertMode::Highlight => if self.highlight == num {
                self.highlight = -1
                } else { self.highlight = num},
            _ => {
                self.moves.push(Move::IMove(self.focus, num));
                self.board.insert(self.focus, num)
                },
        };
        self.insert_mode = InsertMode::Mark;
    }
}

impl View for BoardView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // Initialize board
        match self.helper {
            Helper::Marks => self.board.fill_marks(),
            Helper::On => { self.board.fill_marks(); self.board.solve_singles() },
            Helper::Off => (),
        }


        // Clear the screen
        phi.renderer.set_draw_color(settings::WHITE);
        phi.renderer.clear();



        // Color squares
        for (i, c) in self.square_colores.iter().enumerate() {
            phi.renderer.set_draw_color(*c);
            phi.renderer.fill_rect(Rect::new(11 + 50*(i as i32 % 9),
                11 + 50*(i as i32 /9), 49, 49).unwrap().unwrap());
        }
        // highlight - greyish
        if self.highlight > 0 {
            phi.renderer.set_draw_color(settings::SAND);
            for (i, cell) in self.board.cells.iter().enumerate() {
                if cell.number == self.highlight || (cell.number == 0 && cell.marks[self.highlight as usize] == Mark::Legal) {
                    if self.square_colores[i] == settings::WHITE {
                        phi.renderer.fill_rect(Rect::new(11 + 50*(i as i32 % 9),
                                    11 + 50*(i as i32 /9), 49, 49).unwrap().unwrap());
                    }
                }
            }
        }
        // focus - red outline
        phi.renderer.set_draw_color(::sdl2::pixels::Color::RGB(255, 0, 0));
        phi.renderer.draw_rect(Rect::new(11 + 50*(self.focus as i32 % 9),
            11 + 50*(self.focus as i32 /9), 49, 49).unwrap().unwrap());
        phi.renderer.draw_rect(Rect::new(12 + 50*(self.focus as i32 % 9),
            12 + 50*(self.focus as i32 /9), 47, 47).unwrap().unwrap());




        // Lines
        phi.renderer.set_draw_color(settings::BLACK);
        let outer_box = [Point::new(8,8), Point::new(8, 462), Point::new(462,462),
            Point::new(462, 8), Point::new(8,8), Point::new(9,9), Point::new(9, 461),
            Point::new(461,461), Point::new(461, 9), Point::new(9,9), Point::new(10,10),
            Point::new(10, 460), Point::new(460,460), Point::new(460, 10), Point::new(10,10)];
        let horizontal_lines = [Point::new(10, 60), Point::new(460, 60), Point::new(460, 110),
            Point::new(10, 110), Point::new(10, 159), Point::new(460, 159), Point::new(460, 160),
            Point::new(10, 160), Point::new(10, 161), Point::new(460, 161), Point::new(460, 210),
            Point::new(10, 210), Point::new(10, 260), Point::new(460, 260), Point::new(460, 309),
            Point::new(10, 309), Point::new(10, 310), Point::new(460, 310), Point::new(460, 311),
            Point::new(10, 311), Point::new(10, 360), Point::new(460, 360), Point::new(460, 410),
            Point::new(10, 410), Point::new(10, 460)];
        let vertical_lines = [Point::new(60, 10), Point::new(60, 460), Point::new(110, 460),
            Point::new(110, 10), Point::new(159, 10), Point::new(159, 460), Point::new(160, 460),
            Point::new(160, 10), Point::new(161, 10), Point::new(161, 460), Point::new(210, 460),
            Point::new(210, 10), Point::new(260, 10), Point::new(260, 460), Point::new(309, 460),
            Point::new(309, 10), Point::new(310, 10), Point::new(310, 460), Point::new(311, 460),
            Point::new(311, 10), Point::new(360, 10), Point::new(360, 460), Point::new(410, 460),
            Point::new(410, 10), Point::new(460, 10)];
        phi.renderer.draw_lines(&outer_box);
        phi.renderer.draw_lines(&horizontal_lines);
        phi.renderer.draw_lines(&vertical_lines);

        // big numbers
        for (i, cell) in self.board.cells.iter().enumerate() {
            if cell.number != 0 {
                let text = cell.number.to_string();
                let text = &text[..];
                let surface;
                if cell.read_only {
                    surface = self.fonts.BIGNUM.render(text, ::sdl2_ttf::blended(settings::BLACK)).unwrap();
                } else {
                    surface = self.fonts.BIGNUM.render(text, ::sdl2_ttf::blended(settings::BLUE)).unwrap();
                }
                let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
                phi.renderer.copy(&texture, None, Rect::new(17+(i as i32%9)*50, 17+(i as i32/9)*50, 36, 36).unwrap());
            }
        }
        // small numbers (marks)
        for (i, cell) in self.board.cells.iter().enumerate() {
            for (j, &m) in cell.marks.iter().enumerate() {
                if m == Mark::Legal {
                    let j = j as i32;
                    let text = j.to_string();
                    let text = &text[..];
                    let surface = self.fonts.SMALLNUM.render(text, ::sdl2_ttf::blended(settings::BLACK)).unwrap();
                    let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
                    phi.renderer.copy(&texture, None, Rect::new((j-1)%3*16 + 13 + (i as i32%9)*50,
                                            (j-1)/3*16 + 13 + (i as i32/9)*50, 11, 11).unwrap());
                }
            }
        }



        if phi.events.now.quit || Some(true) == phi.events.now.key_escape {
            return ViewAction::Quit;
        }
        if Some(true) == phi.events.now.key_up || Some(true) == phi.events.now.key_k {
            self.move_focus(-9);
        } if Some(true) == phi.events.now.key_down || Some(true) == phi.events.now.key_j {
            self.move_focus(9);
        } if Some(true) == phi.events.now.key_left || Some(true) == phi.events.now.key_h {
            self.move_focus(-1);
        } if Some(true) == phi.events.now.key_right || Some(true) == phi.events.now.key_l {
            self.move_focus(1);
        }

        // set modes
        if Some(true) == phi.events.now.key_space {
            match self.insert_mode {
                InsertMode::Normal  => self.insert_mode = InsertMode::Mark,
                _                   => self.insert_mode = InsertMode::Normal,
            }
        }
        if Some(true) == phi.events.now.key_r_shift || Some(true) == phi.events.now.key_l_shift {
            self.insert_mode = InsertMode::Highlight;
        }

        // Fill square
        if Some(true) == phi.events.now.key_f {
            if self.highlight > 0 {
                self.board.insert(self.focus, self.highlight);
            }
        }

        // Colores
        if Some(true) == phi.events.now.key_c {
            // Is this slow and unneccesary?
            self.square_colores = [settings::WHITE; 81];
        } 
        if Some(true) == phi.events.now.key_r {
            if self.square_colores[self.focus] == settings::SKIN {
                self.square_colores[self.focus] = settings::WHITE;
            } else {
                self.square_colores[self.focus] = settings::SKIN;
            }
        } if Some(true) == phi.events.now.key_b {
            if self.square_colores[self.focus] == settings::CYAN {
                self.square_colores[self.focus] = settings::WHITE;
            } else {
                self.square_colores[self.focus] = settings::CYAN;
            }
        } if Some(true) == phi.events.now.key_g {
            if self.square_colores[self.focus] == settings::LIME {
                self.square_colores[self.focus] = settings::WHITE;
            } else {
                self.square_colores[self.focus] = settings::LIME;
            }
        } if Some(true) == phi.events.now.key_v {
            if self.square_colores[self.focus] == settings::VIOLET {
                self.square_colores[self.focus] = settings::WHITE;
            } else {
                self.square_colores[self.focus] = settings::VIOLET;
            }
        }

        //undo
        if Some(true) == phi.events.now.key_u {
            self.undo();
        }

        // Number pressed
        if Some(true) == phi.events.now.key_1 {
            self.num_press(1);
        } if Some(true) == phi.events.now.key_2 {
            self.num_press(2);
        } if Some(true) == phi.events.now.key_3 {
            self.num_press(3);
        } if Some(true) == phi.events.now.key_4 {
            self.num_press(4);
        } if Some(true) == phi.events.now.key_5 {
            self.num_press(5);
        } if Some(true) == phi.events.now.key_6 {
            self.num_press(6);
        } if Some(true) == phi.events.now.key_7 {
            self.num_press(7);
        } if Some(true) == phi.events.now.key_8 {
            self.num_press(8);
        } if Some(true) == phi.events.now.key_9 {
            self.num_press(9);
        } 
        // Delete number
        if Some(true) == phi.events.now.key_0 || Some(true) == phi.events.now.key_del {
            self.board.remove(self.focus);
        }

        ViewAction::None
    }
}


fn board_from_file(num: i32) -> Vec<i32> {
    let filename = format!("sudokus/{}.txt", num);
    let mut f = File::open(filename).ok().expect("failed fileopen");
    let mut s = String::new();
    f.read_to_string(&mut s).ok().expect("failed read_to_string");

    s.split_whitespace()
     .map(|s| s.parse().unwrap())
     .collect()
}
