use crate::ecs::component::camera_data::CameraData;
use crate::ecs::component::transform_data::Position;
use crate::ecs::component::{
    collision_data::CollisionData, player_data::PlayerData, render_data::RenderData,
    transform_data, world_data::WorldData,
};
use crate::ecs::entity_manager::EntityManager;
use crate::ecs::system::camera_system;
use crate::ecs::system::{
    collision_system::CollisionSystem, movement_system::MovementSystem,
    player_controller::PlayerController, render_system::RenderSystem, world_system::WorldSystem, camera_system::Camera_System
};
use crate::input_handler::InputHandler;
use crate::sdl_window_manager::SDLWindowManager;

use std::cell::RefCell;
use std::rc::Rc;

use crate::prefabs::{player, test_world};
pub struct Level;

impl Level {
    pub fn load(
        entity_manager: &mut EntityManager,
        input_handler: Rc<RefCell<InputHandler>>,
        window_manager: Rc<RefCell<SDLWindowManager>>,
    ) {
        {
            // Set up systems and wrap them with Rc<RefCell>
            let movement_system = Rc::new(RefCell::new(MovementSystem));
            let render_system = Rc::new(RefCell::new(RenderSystem {
                window_manager: Rc::clone(&window_manager), // Pass the window manager correctly
            }));
            let collision_system = Rc::new(RefCell::new(CollisionSystem));
            let player_controller = Rc::new(RefCell::new(PlayerController {
                input_handler: Rc::clone(&input_handler), // Pass InputHandler correctly
            }));
            let world_system = Rc::new(RefCell::new(WorldSystem {
                window_manager: Rc::clone(&window_manager), // Pass the window manager correctly
            }));
            let player_look = Rc::new(RefCell::new(camera_system::Camera_System {
                window_manager: Rc::clone(&window_manager), // Pass the window manager correctly
            }));

            // Register systems with the entity manager
            entity_manager.add_system(movement_system.clone());
            entity_manager.add_system(render_system.clone());
            entity_manager.add_system(collision_system.clone());
            entity_manager.add_system(player_controller.clone());
            entity_manager.add_system(world_system.clone());
            entity_manager.add_system(player_look.clone());
        }
        // Create a player entity
        player::Player::spawn(entity_manager);
        test_world::TestWorld::spawn(entity_manager, Rc::clone(&window_manager));
    }
}
