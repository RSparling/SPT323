// src/sdl_window_manager.rs

//Description:
//This module will contain the SDLWindowManager struct which will be responsible for managing the SDL window and rendering the game objects.
//This struct will have methods to clear the window, present the window, and draw a square on the window.
//it exposes a new function that takes a Canvas<Window> and returns an SDLWindowManager instance.
//it's also designed to be extensible, so you can add more methods to it in the future.
//
//it is seperte from the ecs module because it is not related to the entity-component-system architecture.
use sdl2::pixels::Color; //for color
use sdl2::rect::Rect; //for rectangle
use sdl2::render::Canvas; //for canvas
use sdl2::video::Window; //for window

pub struct SDLWindowManager {
    //SDLWindowManager struct
    canvas: Canvas<Window>,
}

impl SDLWindowManager {
    //SDLWindowManager struct
    pub fn new(canvas: Canvas<Window>) -> Self {
        SDLWindowManager { canvas }
    }

    pub fn clear(&mut self) {
        //clear window
        self.canvas.set_draw_color(Color::RGB(222, 165, 164));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        //present window
        self.canvas.present();
    }

    pub fn draw_square(&mut self, x: i32, y: i32, size: u32, r: u8, g: u8, b: u8) {
        //draw square
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        let _ = self.canvas.fill_rect(Rect::new(x, y, size, size));
    }
}
