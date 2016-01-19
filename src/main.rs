extern crate sdl2;

mod phi;
mod views;

use ::phi::{Events, Phi, View, ViewAction};

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("RustDoku", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().unwrap()),
        renderer: window.renderer()
            .accelerated()
            .build().unwrap(),
    };

    let mut current_view: Box<View> = Box::new(::views::DefaultView);


    loop {
        context.events.pump();

        match current_view.render(&mut context, 0.01) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
        }
    }
}
