



use ::sdl2::pixels::Color;
use ::sdl2_ttf::Font;
use ::std::path::Path;


pub const WHITE    : Color = Color::RGB(255,255,255);
pub const BLACK    : Color = Color::RGB(0,0,0);
pub const BLUE     : Color = Color::RGB(0,59,235);
pub const OCEAN    : Color = Color::RGB(41,94,255);
pub const SKYE     : Color = Color::RGB(102,140,255);
pub const PURPLE   : Color = Color::RGB(140,102,255);
pub const VIOLET   : Color = Color::RGB(217,102,255);
pub const PINK1    : Color = Color::RGB(255,102,217);
pub const PINK2    : Color = Color::RGB(255,102,140);
pub const YELLOW   : Color = Color::RGB(255,201,41);
pub const ORANGE   : Color = Color::RGB(234,176,0);
pub const CYAN     : Color = Color::RGB(102,255,217);
pub const GREEN    : Color = Color::RGB(102,255,140);
pub const SAND     : Color = Color::RGB(255,217,102);
pub const SKIN     : Color = Color::RGB(255,140,102);
pub const LIME     : Color = Color::RGB(140,255,102);
pub const GRASS    : Color = Color::RGB(217,255,102);

pub struct Fonts {
    pub BIGNUM: Font,
    pub SMALLNUM: Font,
    pub BIGFONT: Font,
    pub SMALLFONT: Font,
}

impl Fonts {
    pub fn new() -> Fonts {
        Fonts {
            BIGNUM: ::sdl2_ttf::Font::from_file(Path::new("assets/arial.ttf"), 60).unwrap(),
            SMALLNUM: ::sdl2_ttf::Font::from_file(Path::new("assets/arial.ttf"), 50).unwrap(),
            BIGFONT: ::sdl2_ttf::Font::from_file(Path::new("assets/arial.ttf"), 60).unwrap(),
            SMALLFONT: ::sdl2_ttf::Font::from_file(Path::new("assets/arial.ttf"), 50).unwrap(),
        }
    }
    
}
   
