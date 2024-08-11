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
use sdl2::Sdl; //for sdl
pub struct SDLWindowManager {
    //SDLWindowManager struct
    canvas: Canvas<Window>,
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    event_pump: Option<sdl2::EventPump>,
}

impl SDLWindowManager {
    pub fn new() -> Self {
        // Initialize SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("SDL Window", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        SDLWindowManager {
            canvas,
            sdl_context,
            video_subsystem,
            event_pump: Some(event_pump), // Store event_pump in Option
        }
    }

    pub fn take_event_pump(&mut self) -> sdl2::EventPump {
        self.event_pump.take().expect("EventPump already taken") // Take event_pump
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(222, 165, 164));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_square(&mut self, x: i32, y: i32, size: u32, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        let _ = self.canvas.fill_rect(Rect::new(x, y, size, size));
    }
}