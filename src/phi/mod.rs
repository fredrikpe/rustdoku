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
    ChangeView(Box<View>),
}

pub trait View {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

struct_events!{
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_space: Space,
        key_left: Left,
        key_right: Right,
        key_del: Delete,
        key_minus: Minus,
        key_plus: Plus,
        key_r_shift: RShift,
        key_l_shift: LShift,
        key_u: U,

        // Fill
        key_f: F,

        // Colores
        key_c: C,
        key_v: V,
        key_b: B,
        key_r: R,
        key_g: G,

        // Move
        key_j: J,
        key_k: K,
        key_l: L,
        key_h: H,

        key_0: Num0,
        key_1: Num1,
        key_2: Num2,
        key_3: Num3,
        key_4: Num4,
        key_5: Num5,
        key_6: Num6,
        key_7: Num7,
        key_8: Num8,
        key_9: Num9

    },
    else: {
        quit: Quit { .. }
    }
}

pub fn spawn<F>(title: &str, init: F)
where F: Fn(&mut Phi) -> Box<View> {
    // Initialize SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _ttf_context = ::sdl2_ttf::init();

    // Create the window
    let window = video.window(title, 800, 600)
    .position_centered().opengl()
    .build().unwrap();

    // Create the context
    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().unwrap()),
        renderer: window.renderer()
        .accelerated()
        .build().unwrap()
    };

    // Create the default view
    let mut current_view = init(&mut context);


    // Frame timing

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;



    loop {
        // Frame timing (bis)

        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            //println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        // Logic & rendering

        context.events.pump();

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view,
        }
    }
}
