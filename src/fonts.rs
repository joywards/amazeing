use std::env;
use std::path::PathBuf;
use sdl2::ttf::Sdl2TtfContext;


pub type Font = sdl2::ttf::Font<'static, 'static>;

pub struct Fonts {
    pub default: Font,
    pub small: Font,
}

impl Fonts {
    pub fn new(context: &'static Sdl2TtfContext) -> Fonts {
        let dir = Self::find_fonts_dir();
        Self {
            default: context.load_font(dir.join("mystyle.ttf"), 64).unwrap(),
            small: context.load_font(dir.join("mystyle.ttf"), 30).unwrap(),
        }
    }

    fn find_fonts_dir() -> PathBuf {
        for dir in env::var("CARGO_MANIFEST_DIR").iter()
            .chain([".".to_string(), "..".to_string()].iter())
        {
            let fonts_dir = [dir, "fonts"].iter().collect::<PathBuf>();
            if fonts_dir.is_dir() {
                return fonts_dir;
            }
        }
        panic!("Could not find a directory with fonts");
    }
}
