mod ecs;
mod input_handler;
mod sdl_window_manager;
mod level;
mod game_manager;

use game_manager::GameManager;
use input_handler::InputHandler;
use sdl_window_manager::SDLWindowManager;
use sdl2::Sdl;

fn main() -> Result<(), String> {
    // Initialize SDL
    let sdl_context: Sdl = sdl2::init()?;
    
    // Create an SDL window manager
    let window_manager = SDLWindowManager::builder()
        .width(800)
        .height(600)
        .title("Game Window")
        .build();

    // Create an input handler
    let event_pump = sdl_context.event_pump()?;
    let input_handler = InputHandler::new(event_pump);

    // Initialize GameManager with the created input handler and window manager
    let game_manager = GameManager::new(input_handler, window_manager);

    // Initialize the level and run the game loop
    game_manager.borrow_mut().initialize_level();
    game_manager.borrow_mut().run_game_loop();

    Ok(())
}
