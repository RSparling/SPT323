mod ecs;
mod input_handler;
mod sdl_window_manager;

use ecs::component::{
    collision_data::CollisionData, player_data::PlayerData, position_data::Position,
    render_data::RenderData, velocity_data::Velocity,
};
use ecs::entity_manager::EntityManager;
use ecs::system::{
    collision_system::CollisionSystem, movement_system::MovementSystem, player_controller::PlayerController,
    render_system::RenderSystem,
};
use input_handler::InputHandler;
use sdl_window_manager::SDLWindowManager;

use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), String> {
    // Use the builder to create the window manager
    let window_manager = SDLWindowManager::builder()
        .width(800)
        .height(600)
        .title("Game Window")
        .build();
    
    let event_pump = sdl2::init()?.event_pump()?; // Initialize EventPump directly
    let input_handler = Rc::new(RefCell::new(InputHandler::new(event_pump))); // Wrap InputHandler in Rc<RefCell>

    let mut entity_manager = EntityManager::new(); // Create entity manager

    // Set up systems and wrap them with Rc<RefCell>
    let movement_system = Rc::new(RefCell::new(MovementSystem));
    let render_system = Rc::new(RefCell::new(RenderSystem {
        window_manager: Rc::new(RefCell::new(window_manager)), // Pass the window manager correctly
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
    entity_manager.add_component(&player_entity, Position { x: 50.0, y: 50.0 });
    entity_manager.add_component(
        &player_entity,
        RenderData {
            r: 0.99,
            g: 0.99,
            b: 0.5,
            size: 10.0,
        },
    );
    entity_manager.add_component(&player_entity, Velocity { x: 0.0, y: 0.0 });
    entity_manager.add_component(&player_entity, PlayerData {});

    entity_manager.register_entity_to_system::<MovementSystem>(&player_entity);
    entity_manager.register_entity_to_system::<RenderSystem>(&player_entity);
    entity_manager.register_entity_to_system::<PlayerController>(&player_entity);

    // Create 50 entities with random positions and velocities
    for _ in 0..50 {
        let entity = entity_manager.create_entity();
        entity_manager.add_component(
            &entity,
            Position {
                x: rand::random::<f32>() * 800.0,
                y: rand::random::<f32>() * 600.0,
            },
        );
        entity_manager.add_component(
            &entity,
            RenderData {
                r: rand::random::<f32>(),
                g: rand::random::<f32>(),
                b: rand::random::<f32>(),
                size: 50.0,
            },
        );
        entity_manager.add_component(
            &entity,
            Velocity {
                x: rand::random::<f32>() * 10.0 - 5.0,
                y: rand::random::<f32>() * 10.0 - 5.0,
            },
        );
        entity_manager.add_component(&entity, CollisionData {});

        entity_manager.register_entity_to_system::<MovementSystem>(&entity);
        entity_manager.register_entity_to_system::<RenderSystem>(&entity);
        entity_manager.register_entity_to_system::<CollisionSystem>(&entity);
    }

    // Game loop
    'running: loop {
        input_handler.borrow_mut().update(); // Update input handler

        if input_handler.borrow().is_key_down(Keycode::Escape) {
            break 'running; // Exit the game loop if Escape is pressed
        }

        entity_manager.update(); // Update all systems through the entity manager

        // Sleep to limit frame rate to 60 FPS
        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
