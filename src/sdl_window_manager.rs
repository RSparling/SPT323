use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SDLWindowManager {
    canvas: Rc<RefCell<Canvas<Window>>>,
    window_width: u32,
    window_height: u32,
}

impl SDLWindowManager {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>, width: u32,height: u32) -> Self {
        SDLWindowManager { canvas, window_width: width, window_height: height }
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

    pub fn draw_filled_rect(&self, x: i32, y: i32, size_x: u32, size_y: u32, r: u8, g: u8, b: u8) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(r, g, b));
        let _ = canvas.fill_rect(Rect::new(x, y, size_x, size_y));
    }
    
    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, r: u8, g: u8, b: u8) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(r, g, b));
        let _ = canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2));
    }

    pub fn draw_rect(&self, x: i32, y: i32, width: u32, height: u32, r: u8, g: u8, b: u8) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(r, g, b));
        let _ = canvas.draw_rect(Rect::new(x, y, width, height));
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        let canvas = self.canvas.borrow();
        let size = canvas.output_size().unwrap();
        (size.0, size.1)
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

        SDLWindowManager::new(canvas, self.width, self.height)
    }
}

impl Default for SDLWindowManagerBuilder {
    fn default() -> Self {
        SDLWindowManagerBuilder {
            width: 800,
            height: 800,
            title: "SDL Window".to_string(),
        }
    }
}
