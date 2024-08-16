use crate::ecs::entity_manager::EntityManager;
use crate::input_handler::InputHandler;
use crate::level::level::Level;
use crate::sdl_window_manager::SDLWindowManager;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub struct GameManager {
    pub input_handler: Rc<RefCell<InputHandler>>,
    pub window_manager: Rc<RefCell<SDLWindowManager>>,
    pub entity_manager: Option<EntityManager>,
}

impl GameManager {
    pub fn new(input_handler: InputHandler, window_manager: SDLWindowManager) -> Rc<RefCell<Self>> {
        let input_handler_rc = Rc::new(RefCell::new(input_handler));
        let window_manager_rc = Rc::new(RefCell::new(window_manager));

        // First, create a GameManager without the EntityManager
        let game_manager_rc = Rc::new(RefCell::new(GameManager {
            input_handler: Rc::clone(&input_handler_rc),
            window_manager: Rc::clone(&window_manager_rc),
            entity_manager: None, // Start with None
        }));

        // Now, create the EntityManager and assign it to the GameManager
        let entity_manager = EntityManager::new(Rc::clone(&game_manager_rc));
        game_manager_rc.borrow_mut().entity_manager = Some(entity_manager);

        // Return the Rc<RefCell<GameManager>>
        game_manager_rc
    }

    pub fn get_entity_manager_mut(&mut self) -> &mut EntityManager {
        self.entity_manager
            .as_mut()
            .expect("EntityManager is not initialized")
    }

    pub fn initialize_level(&mut self) {
        let input_handler = Rc::clone(&self.input_handler);
        let window_manager = Rc::clone(&self.window_manager);

        // Load the level (entities and systems)
        Level::load(self.get_entity_manager_mut(), input_handler, window_manager);
    }

    pub fn run_game_loop(&mut self) {
        'running: loop {
            std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");
    
            //clear window
            {
                self.window_manager.borrow_mut().clear();
            }
            self.input_handler.borrow_mut().update();

            if self.input_handler.borrow().is_key_down(Keycode::Escape) {
                break 'running; // Exit the game loop if Escape is pressed
            }

            self.get_entity_manager_mut().update(); // Update all systems through the entity manager

            //present the window
            {
                self.window_manager.borrow_mut().present();
            }
            // Sleep to limit frame rate to 60 FPS
            std::thread::sleep(Duration::from_millis(6));
        }
    }
}
