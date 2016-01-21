
use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;
use ::sdl2::rect::{Point, Rect};
use ::std::path::Path;

pub struct MenuView;

pub struct BoardView {
    cells: [Cell; 81],
}


impl BoardView {
    pub fn new(phi: &mut Phi) -> BoardView {
        BoardView {
            cells: [Cell::new(); 81],
        }
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

        // Render the scene
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        let mut outer_box = [Point::new(10,10), Point::new(10, 460), Point::new(460,460),
            Point::new(460, 10), Point::new(10,10)];
        let mut horizontal_lines = [Point::new(10, 60), Point::new(460, 60), Point::new(460, 110),
            Point::new(10, 110), Point::new(10, 160), Point::new(460, 160), Point::new(460, 210),
            Point::new(10, 210), Point::new(10, 260), Point::new(460, 260), Point::new(460, 310),
            Point::new(10, 310), Point::new(10, 360), Point::new(460, 360), Point::new(460, 410),
            Point::new(10, 410), Point::new(10, 460)];
        let mut vertical_lines = [Point::new(60, 10), Point::new(60, 460), Point::new(110, 460),
            Point::new(110, 10), Point::new(160, 10), Point::new(160, 460), Point::new(210, 460),
            Point::new(210, 10), Point::new(260, 10), Point::new(260, 460), Point::new(310, 460),
            Point::new(310, 10), Point::new(360, 10), Point::new(360, 460), Point::new(410, 460),
            Point::new(410, 10), Point::new(460, 10)];
        phi.renderer.draw_lines(&outer_box);
        phi.renderer.draw_lines(&horizontal_lines);
        phi.renderer.draw_lines(&vertical_lines);

        // Render numbers
        let font = ::sdl2_ttf::Font::from_file(Path::new("assets/belligerent.ttf"), 30).unwrap();
        let surface = font.render("Hallo", ::sdl2_ttf::blended(Color::RGB(0, 0, 0))).unwrap();
        // Then, we turn this image into a `Texture`, which is more efficient.
        let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
        phi.renderer.copy(&texture, None, Rect::new(10, 10, 60, 30).unwrap());







        if phi.events.now.quit || Some(true) == phi.events.now.key_escape {
            return ViewAction::Quit;
        }

        ViewAction::None
    }
}

#[derive(Clone, Copy)]
struct Cell {
    marks: [bool; 9],
    number: i32,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            marks: [false; 9],
            number: 0,
        }
    }
}
