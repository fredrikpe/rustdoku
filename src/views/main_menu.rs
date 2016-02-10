
use phi::{Phi, View, ViewAction};
use ::sdl2::rect::{Point, Rect};

use ::views::settings;

struct Action {
    func: Box<Fn(&mut Phi) -> ViewAction>,
    label: &'static str,
}

pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,
    fonts: settings::Fonts,
}
impl Action {
    fn new(phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi) -> ViewAction>) -> Action {
        Action {
            func: func,
            label: label,
        }
    }
}

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        MainMenuView {
            actions: vec![
                Action::new(phi, "New Game", Box::new(|phi| {
                    ViewAction::ChangeView(Box::new(::views::board_view::BoardView::new(phi, 2)))
                })),
                Action::new(phi, "Quit", Box::new(|phi| {
                    ViewAction::Quit
                })),
            ],
            selected: 0,
            fonts: settings::Fonts::new(),
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }


        // Execute the currently selected option.
        if phi.events.now.key_space == Some(true) {
            return (self.actions[self.selected as usize].func)(phi);
        }

        // Change the selected action using the keyboard.
        if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }

        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }


        // Clear the screen.
        phi.renderer.set_draw_color(settings::BLACK);
        phi.renderer.clear();


        for (i, action) in self.actions.iter().enumerate() {
            if self.selected as usize == i {
                let surface = self.fonts.BIGFONT.render(action.label, ::sdl2_ttf::blended(settings::WHITE)).unwrap();
                
                let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
                phi.renderer.copy(&texture, None, Rect::new(100, 100, 200, 200).unwrap());
            } else {
                let surface = self.fonts.BIGFONT.render(action.label, ::sdl2_ttf::blended(settings::WHITE)).unwrap();
                let texture = phi.renderer.create_texture_from_surface(&surface).unwrap();
                phi.renderer.copy(&texture, None, Rect::new(200, 300, 100, 100).unwrap());
            }
        }

        ViewAction::None
    }
}
