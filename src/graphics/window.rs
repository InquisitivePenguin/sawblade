extern crate sdl2;

pub struct Window {
    title: String,
    sdl_win: sdl2::video::Window
}

impl Window {
    pub fn new(res: (u32,u32), title: String) -> Window {
        let context = sdl2::init().unwrap();
        let vid = context.video().unwrap();
        let window = vid.window(title.as_str(),
            res.0,
            res.1
        ).position_centered().build().unwrap();
        Window {
            title: title,
            sdl_win: window,
        }
    }
    pub fn close(w: Window) {
        w.sdl_win; // Capture window and make it go out of scope
    }
}