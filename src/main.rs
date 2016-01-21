
extern crate sdl2;
extern crate sdl2_ttf;


mod phi;
mod views;
mod logic;


fn main() {
    ::phi::spawn("RustDoku", |phi| {
        Box::new(::views::BoardView::new(phi))
    });
}
