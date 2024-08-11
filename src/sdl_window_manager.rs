use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SDLWindowManager {
    canvas: Rc<RefCell<Canvas<Window>>>,
}

impl SDLWindowManager {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Self {
        SDLWindowManager { canvas }
    }

    pub fn builder() -> SDLWindowManagerBuilder {
        SDLWindowManagerBuilder::default()
    }

    pub fn clear(&self) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(222, 165, 164));
        canvas.clear();
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }

    pub fn draw_square(&self, x: i32, y: i32, size: u32, r: u8, g: u8, b: u8) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(r, g, b));
        let _ = canvas.fill_rect(Rect::new(x, y, size, size));
    }
}

pub struct SDLWindowManagerBuilder {
    width: u32,
    height: u32,
    title: String,
}

impl SDLWindowManagerBuilder {
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn build(self) -> SDLWindowManager {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = Rc::new(RefCell::new(window.into_canvas().build().unwrap()));

        SDLWindowManager::new(canvas)
    }
}

impl Default for SDLWindowManagerBuilder {
    fn default() -> Self {
        SDLWindowManagerBuilder {
            width: 800,
            height: 600,
            title: "SDL Window".to_string(),
        }
    }
}

