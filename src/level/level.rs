use crate::ecs::component::{
    collision_data::CollisionData, player_data::PlayerData, transform_data,
    render_data::RenderData,
};
use crate::ecs::entity_manager::EntityManager;
use crate::ecs::system::{
    collision_system::CollisionSystem, movement_system::MovementSystem, player_controller::PlayerController,
    render_system::RenderSystem,
};
use crate::input_handler::InputHandler;
use crate::sdl_window_manager::SDLWindowManager;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Level;

impl Level {
    pub fn load(entity_manager: &mut EntityManager, input_handler: Rc<RefCell<InputHandler>>, window_manager: Rc<RefCell<SDLWindowManager>>) {
        // Set up systems and wrap them with Rc<RefCell>
        let movement_system = Rc::new(RefCell::new(MovementSystem));
        let render_system = Rc::new(RefCell::new(RenderSystem {
            window_manager: Rc::clone(&window_manager), // Pass the window manager correctly
        }));
        let collision_system = Rc::new(RefCell::new(CollisionSystem));
        let player_controller = Rc::new(RefCell::new(PlayerController {
            input_handler: Rc::clone(&input_handler), // Pass InputHandler correctly
        }));

        // Register systems with the entity manager
        entity_manager.add_system(movement_system.clone());
        entity_manager.add_system(render_system.clone());
        entity_manager.add_system(collision_system.clone());
        entity_manager.add_system(player_controller.clone());

        // Create a player entity
        let player_entity = entity_manager.create_entity();
        entity_manager.add_component(
            &player_entity,
            RenderData {
                r: 0.99,
                g: 0.99,
                b: 0.5,
                size: 10.0,
            },
        );
        entity_manager.add_component(&player_entity, transform_data::Transform { velocity: transform_data::Velocity { delta_x: 0.0, delta_y: 0.0 }, position: transform_data::Position { pos_x: 50.0, pos_y: 50.0, rotation:0.0} });
        entity_manager.add_component(&player_entity, PlayerData {});

        entity_manager.register_entity_to_system::<MovementSystem>(&player_entity);
        entity_manager.register_entity_to_system::<RenderSystem>(&player_entity);
        entity_manager.register_entity_to_system::<PlayerController>(&player_entity);
    }
}
