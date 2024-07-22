// Russell Sparling
// July 21, 2024
// Moving graphics in Rust using SDL2
//
// Descrption:
// This program creates a window using SDL2 and moves a square around the screen. The square can be moved using the WASD keys.
// The program also implements the start of an entity-component-system architecture to manage game entities and their components.
// Program Structure:
// - ECS (Entity-Component-System) module: Contains the entity manager, components, and systems for the game.
//   -- Components: Position and RenderData components that can be attached to entities.
//   -- Entity Manager: Manages entities and their components.
//   -- Systems: MovementSystem and RenderSystem that update entities based on their components.
// - Input Handler module: Handles input from the user. It checks for quit events and key presses.
// - SDL Window Manager module: Manages the SDL window and rendering of game objects.
//
// Areas for Improvement:
// - Current ECS manager implementation is poorly optimized with a much higher comlexity than necessary.
// - The ECS manager should be refactored to use a more efficient data structure for storing entities and components.
// - SDL should be refined more to have window management and rendering seperate for the implementation of DDA raycasting.
// - The input handler should be refactored to handle more complex input events and key combinations.
// - Systems and Components should be refactored to be submodules of the ECS module. This way it will be easier to add more systems and components in the future.
// - Service/system locator pattern should be implemented to allow for more flexible system management. This will allow for easier addition and removal of systems
//   especially ones unrelated to the ECS architecture.
//
// Other Notes:
// I know this is overkill for a simple square moving around the screen, but I build something similar in c++ and wanted to see if I could do it in Rust.

mod ecs;
mod input_handler;
mod sdl_window_manager;
//mod ecs;
use ecs::components::{Position, RenderData};
use ecs::entity_manager::EntityManager;
use ecs::systems::{MovementSystem, RenderSystem, System};
use input_handler::InputHandler;
use sdl_window_manager::SDLWindowManager;
//use ecs::components::{Position, RenderData};
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context: Sdl = sdl2::init()?; //initialize sdl
    let video_subsystem = sdl_context.video()?; //initialize video subsystem

    let window = video_subsystem
        .window("SDL Window", 800, 600) //create window
        .position_centered() //center window
        .build() //build window
        .map_err(|e| e.to_string())?; //error handling

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?; //create canvas
    let mut window_manager = SDLWindowManager::new(canvas); //create window manager

    let mut entity_manager = EntityManager::new(); //create entity manager

    let entity = entity_manager.create_entity(); //create entity
    entity_manager.add_component(&entity, Position { x: 50.0, y: 50.0 }); //add position component
    entity_manager.add_component(
        &entity,
        RenderData {
            r: 0.99,
            g: 0.99,
            b: 0.5,
            size: 10.0,
        },
    ); //add render data component

    let mut movement_system = MovementSystem; //create movement system
    let mut render_system = RenderSystem {
        window_manager: &mut window_manager,
    }; //create render system

    let event_pump = sdl_context.event_pump()?; //create event pump
    let mut input_handler = InputHandler::new(event_pump); // create input handler

    // Game loop
    'running: loop {
        //game loop, the 'running is a label
        input_handler.update(); // update input handler

        if input_handler.is_key_down(Keycode::Escape) {
            //if escape key is pressed
            break 'running; //break out of loop
        }

        movement_system.update(&mut entity_manager, &input_handler); //update movement system
        render_system.update(&mut entity_manager); //update render system

        // Sleep to limit frame rate
        std::thread::sleep(Duration::from_millis(16)); //sleep for 16 milliseconds, 60 fps
    }

    Ok(()) //return ok
}
