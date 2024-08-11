//src/input_handler.rs
//Description: Handles input from the user. It checks for quit events and key presses.
//It uses the sdl2 crate to handle input events.

use sdl2::event::Event; // for events
use sdl2::keyboard::{Keycode, Scancode}; // for keycodes and scancodes

pub struct InputHandler<'a> {
    // InputHandler struct
    event_pump: &'a mut sdl2::EventPump, // EventPump is now a mutable reference
}


impl<'a> InputHandler<'a> {
    // InputHandler implementation
    pub fn new(event_pump: &'a mut sdl2::EventPump) -> Self {
        InputHandler { event_pump }
    }

    pub fn update(&mut self) {
        // Check for quit event
        for event in self.event_pump.poll_iter() {

            // Poll for events
            match event {
                Event::Quit { .. } => std::process::exit(0), // Quit if the event is a quit event
                _ => {}
            }
        }
        // update keystates if needed in the future
    }

    pub fn is_key_down(&self, key: Keycode) -> bool {
        // Check if a key is pressed
        if let Some(scancode) = Scancode::from_keycode(key) {
            // Convert Keycode to Scancode
            self.event_pump
                .keyboard_state()
                .is_scancode_pressed(scancode) // Check if the scancode is pressed
        } else {
            false // Return false if conversion fails
        }
    }

    // Check specific keys
    pub fn is_w_pressed(&self) -> bool {
        self.is_key_down(Keycode::W)
    }

    pub fn is_a_pressed(&self) -> bool {
        self.is_key_down(Keycode::A)
    }

    pub fn is_s_pressed(&self) -> bool {
        self.is_key_down(Keycode::S)
    }

    pub fn is_d_pressed(&self) -> bool {
        self.is_key_down(Keycode::D)
    }
}
