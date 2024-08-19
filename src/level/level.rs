use crate::ecs::component::transform_data::Position;
use crate::ecs::component::{
    collision_data::CollisionData, player_data::PlayerData, render_data::RenderData,
    transform_data, world_data::WorldData,
};
use crate::ecs::entity_manager::EntityManager;
use crate::ecs::system::player_look;
use crate::ecs::system::{
    collision_system::CollisionSystem, movement_system::MovementSystem,
    player_controller::PlayerController, render_system::RenderSystem, world_system::WorldSystem, player_look::PlayerLook
};
use crate::input_handler::InputHandler;
use crate::sdl_window_manager::SDLWindowManager;

use std::cell::RefCell;
use std::rc::Rc;

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
            let player_look = Rc::new(RefCell::new(player_look::PlayerLook {
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
        entity_manager.add_component(
            &player_entity,
            transform_data::Transform {
                velocity: transform_data::Velocity {
                    delta_x: 0.0,
                    delta_y: 0.0,
                },
                position: Position::new(100.0, 100.0, 0.0),
            },
        );
        entity_manager.add_component(&player_entity, PlayerData::new());

        let world_entity = entity_manager.create_entity();
        let window_size_x;
        let window_size_y;
        {
            let sdl_window_manager = window_manager.borrow_mut();
            let (window_width, window_height) = sdl_window_manager.get_window_size();
            window_size_x = window_width;
            window_size_y = window_height;
        }
        entity_manager.add_component(
            &world_entity,
            WorldData::new(20, window_size_x, window_size_y),
        );

        entity_manager.register_entity_to_system::<MovementSystem>(&player_entity);
        entity_manager.register_entity_to_system::<PlayerController>(&player_entity);
        entity_manager.register_entity_to_system::<PlayerLook>(&player_entity);
        entity_manager.register_entity_to_system::<WorldSystem>(&world_entity);
    }
}
