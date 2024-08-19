//src/input_handler.rs
//Description: Handles input from the user. It checks for quit events and key presses.
//It uses the sdl2 crate to handle input events.

use std::cell::RefCell;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

pub struct InputHandler {
    event_pump: RefCell<sdl2::EventPump>, // EventPump is now wrapped in RefCell
}

impl InputHandler {
    // No longer returns Rc<Self>, just Self
    pub fn new(event_pump: sdl2::EventPump) -> Self {
        InputHandler {
            event_pump: RefCell::new(event_pump),
        }
    }

    pub fn update(&self) {
        let mut event_pump = self.event_pump.borrow_mut();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                _ => {}
            }
        }
    }

    pub fn is_key_down(&self, key: Keycode) -> bool {
        if let Some(scancode) = Scancode::from_keycode(key) {
            self.event_pump
                .borrow()
                .keyboard_state()
                .is_scancode_pressed(scancode)
        } else {
            false
        }
    }

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

    pub fn is_q_pressed(&self) -> bool {
        self.is_key_down(Keycode::Q)
    }

    pub fn is_e_pressed(&self) -> bool {
        self.is_key_down(Keycode::E)
    }
}
