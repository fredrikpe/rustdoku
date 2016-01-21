
use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;
use ::sdl2::rect::{Point, Rect};
use ::std::path::Path;
use ::logic::{Board, Cell};

pub struct MenuView;

pub struct BoardView {
    board: Board,
    focus: i32,
}


impl BoardView {
    pub fn new(phi: &mut Phi) -> BoardView {
        BoardView {
            board: Board::new(),
            focus: 0,
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

}

impl View for BoardView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }


        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(255, 255, 255));
        phi.renderer.clear();

        // highlight focus
        phi.renderer.set_draw_color(Color::RGB(200, 200, 75));
        phi.renderer.fill_rect(Rect::new(10 + 50*(self.focus % 9), 10 + 50*(self.focus/9), 50, 50).unwrap().unwrap());
        // Render the scene
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        let outer_box = [Point::new(10,10), Point::new(10, 460), Point::new(460,460),
            Point::new(460, 10), Point::new(10,10), Point::new(11,11), Point::new(11, 459),
            Point::new(459,459), Point::new(459, 11), Point::new(11,11), Point::new(12,12),
            Point::new(12, 458), Point::new(458,458), Point::new(458, 12), Point::new(12,12)];
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
        let font = ::sdl2_ttf::Font::from_file(Path::new("assets/belligerent.ttf"), 90).unwrap();
        for (i, cell) in self.board.cells.iter().enumerate() {
            if cell.number != 0 {
                let text = cell.number.to_string();
                let text = &text[..];
                let surface = font.render(text, ::sdl2_ttf::blended(Color::RGB(0, 0, 0))).unwrap();
                let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
                phi.renderer.copy(&texture, None, Rect::new(20+(i as i32%9)*50, 20+(i as i32/9)*50, 30, 30).unwrap());
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

        if Some(true) == phi.events.now.key_1 {
            self.board.insert(self.focus as usize, 1);
        } if Some(true) == phi.events.now.key_2 {
            self.board.insert(self.focus as usize, 2);
        } if Some(true) == phi.events.now.key_3 {
            self.board.insert(self.focus as usize, 3);
        } if Some(true) == phi.events.now.key_4 {
            self.board.insert(self.focus as usize, 4);
        } if Some(true) == phi.events.now.key_5 {
            self.board.insert(self.focus as usize, 5);
        } if Some(true) == phi.events.now.key_6 {
            self.board.insert(self.focus as usize, 6);
        } if Some(true) == phi.events.now.key_7 {
            self.board.insert(self.focus as usize, 7);
        } if Some(true) == phi.events.now.key_8 {
            self.board.insert(self.focus as usize, 8);
        } if Some(true) == phi.events.now.key_9 {
            self.board.insert(self.focus as usize, 9);
        } if Some(true) == phi.events.now.key_0 || Some(true) == phi.events.now.key_del {
            self.board.remove(self.focus as usize);
        }

        ViewAction::None
    }
}
