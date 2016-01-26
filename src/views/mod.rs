
use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;
use ::sdl2::rect::{Point, Rect};
use ::std::path::Path;
use ::logic::{Board, Cell};

pub struct MenuView;

enum Mode {
    Normal      = 0,
    Mark        = 1,
    Highlight   = 2,
}


pub struct BoardView {
    board: Board,
    focus: usize,
    highlight: i32,
    colored: Vec<(usize, i32)>,
    mode: Mode,
}


impl BoardView {
    pub fn new(phi: &mut Phi, board: i32) -> BoardView {
        BoardView {
            board: Board::new(),
            focus: 0,
            highlight: -1,
            colored: Vec::new(),
            mode: Mode::Normal,
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
    fn num_press(&mut self, num: i32) {
        match self.mode {
            Mode::Mark => self.board.mark(self.focus, num as usize),
            Mode::Highlight => if self.highlight == num {
                self.highlight = -1
                } else { self.highlight = num},
            _ => self.board.insert(self.focus, num),
        };
        self.mode = Mode::Normal;
    }
}

impl View for BoardView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }


        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(255, 255, 255));
        phi.renderer.clear();


        // highlight - greyish
        if self.highlight > 0 {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 200));
            for (i, cell) in self.board.cells.iter().enumerate() {
                if cell.number == self.highlight || (cell.number == 0 && cell.marks[self.highlight as usize]) {
                    phi.renderer.fill_rect(Rect::new(10 + 50*(i as i32 % 9),
                                10 + 50*(i as i32 /9), 50, 50).unwrap().unwrap());
                }
            }
        }
        // focus - red outline
        phi.renderer.set_draw_color(Color::RGB(255, 0, 0));
        phi.renderer.draw_rect(Rect::new(11 + 50*(self.focus as i32 % 9),
            11 + 50*(self.focus as i32 /9), 49, 49).unwrap().unwrap());
        phi.renderer.draw_rect(Rect::new(12 + 50*(self.focus as i32 % 9),
            12 + 50*(self.focus as i32 /9), 47, 47).unwrap().unwrap());
        // colored


        // Lines
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
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
        let font = ::sdl2_ttf::Font::from_file(Path::new("assets/arial.ttf"), 60).unwrap();
        for (i, cell) in self.board.cells.iter().enumerate() {
            if cell.number != 0 {
                let text = cell.number.to_string();
                let text = &text[..];
                let surface = font.render(text, ::sdl2_ttf::blended(Color::RGB(0, 0, 0))).unwrap();
                let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
                phi.renderer.copy(&texture, None, Rect::new(17+(i as i32%9)*50, 17+(i as i32/9)*50, 36, 36).unwrap());
            }
        }
        // small numbers (marks)
        let font = ::sdl2_ttf::Font::from_file(Path::new("assets/arial.ttf"), 60).unwrap();
        for (i, cell) in self.board.cells.iter().enumerate() {
            for (j, &b) in cell.marks.iter().enumerate() {
                if b {
                    let j = j as i32;
                    let text = j.to_string();
                    let text = &text[..];
                    let surface = font.render(text, ::sdl2_ttf::blended(Color::RGB(0, 0, 0))).unwrap();
                    let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
                    phi.renderer.copy(&texture, None, Rect::new((j-1)%3*16 + 13 + (i as i32%9)*50,
                                            (j-1)/3*16 + 13 + (i as i32/9)*50, 11, 11).unwrap());
                }
            }
        }



        if phi.events.now.quit || Some(true) == phi.events.now.key_escape {
            return ViewAction::Quit;
        }
        if Some(true) == phi.events.now.key_up {
            self.move_focus(-9);
        } if Some(true) == phi.events.now.key_down {
            self.move_focus(9);
        } if Some(true) == phi.events.now.key_left {
            self.move_focus(-1);
        } if Some(true) == phi.events.now.key_right {
            self.move_focus(1);
        }

        // set modes
        if Some(true) == phi.events.now.key_minus || Some(true) == phi.events.now.key_plus {
            self.mode = Mode::Mark;
        }
        if Some(true) == phi.events.now.key_r_shift || Some(true) == phi.events.now.key_l_shift {
            self.mode = Mode::Highlight;
        }

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
        } if Some(true) == phi.events.now.key_0 || Some(true) == phi.events.now.key_del {
            self.board.remove(self.focus);
        }

        ViewAction::None
    }
}

/*
fn orSome(a: Option<bool>, b: Option<bool>) -> Option<bool> {
    match a {
        None => None,
        Some(a_val) => match b {
            None => Some(a_val),
            Some(b_val) => Some(a_val || b_val)
        }
    }
}*/
