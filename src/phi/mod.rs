// phi/mod.rs

#[macro_use]
mod events;

use ::sdl2::render::Renderer;

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

struct_events!{
    keyboard: {
        key_escape: Escape
    },
    else: {
        quit: Quit { .. }
    }
}
