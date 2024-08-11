mod ecs;
mod input_handler;
mod sdl_window_manager;
use ecs::component::position_data;
//mod ecs;
use ecs::component::{collision_data::CollisionData, player_data::PlayerData, position_data::Position, render_data::RenderData, velocity_data::Velocity};
use ecs::entity_manager::EntityManager;
use ecs::system::{collision_system::CollisionSystem, movement_system::MovementSystem, player_controller::PlayerController, render_system::RenderSystem, System};
use input_handler::InputHandler;
use sdl_window_manager::SDLWindowManager;
//use ecs::components::{Position, RenderData};
use sdl2::keyboard::Keycode;

use std::time::Duration;
//other
use rand::Rng;
fn main() -> Result<(), String> {
    let mut window_manager = SDLWindowManager::new(); //create window manager
    
    // Move event_pump out of window_manager, if possible
    let mut event_pump = window_manager.take_event_pump(); 

    // Proceed with setting up systems
    let mut input_handler = InputHandler::new(&mut event_pump);
    let mut entity_manager = EntityManager::new(); //create entity manager

    let entity = entity_manager.create_entity(); //create entity
    entity_manager.add_component(&entity, position_data::Position { x: 50.0, y: 50.0 }); //add position component
    entity_manager.add_component(
        &entity,
        RenderData {
            r: 0.99,
            g: 0.99,
            b: 0.5,
            size: 10.0,
        },
    ); //add render data component
    entity_manager.add_component(&entity, Velocity { x: 0.0, y: 0.0 }); //add velocity component
    //add player data component to mark the entity as a player
    entity_manager.add_component(&entity, PlayerData {});
    let mut movement_system = MovementSystem; //create movement system
    let mut render_system = RenderSystem {
        window_manager: &mut window_manager,
    }; //create render system
    let mut collision_system = CollisionSystem{}; //create collision system    
    let mut player_controller = PlayerController {}; //create player controller
    //create 10 entities with random positions and velocities
    for _ in 0..50 {
        let entity = entity_manager.create_entity(); //create entity
        entity_manager.add_component(
            &entity,
            Position {
                x: rand::random::<f32>() * 800.0,
                y: rand::random::<f32>() * 600.0,
            },
        ); //add position component
        entity_manager.add_component(
            &entity,
            RenderData {
                r: rand::random::<f32>(),
                g: rand::random::<f32>(),
                b: rand::random::<f32>(),
                size: 50.0,
            },
        ); //add render data component
        entity_manager.add_component(
            &entity,
            Velocity {
                x: rand::random::<f32>() * 10.0 - 5.0,
                y: rand::random::<f32>() * 10.0 - 5.0,
            },
        ); //add velocity component
        //add collision data component to mark the entity as a collidable object
        entity_manager.add_component(&entity, CollisionData {});
    }


    // Game loop
    'running: loop {
        //game loop, the 'running is a label
        input_handler.update(); // update input handler

        if input_handler.is_key_down(Keycode::Escape) {
            //if escape key is pressed
            break 'running; //break out of loop
        }

        player_controller.update(&mut entity_manager, &input_handler); //update player controller
        movement_system.update(&mut entity_manager); //update movement system
        collision_system.update(&mut entity_manager); //update collision system
        render_system.update(&mut entity_manager); //update render system
        
        // Sleep to limit frame rate
        std::thread::sleep(Duration::from_millis(16)); //sleep for 16 milliseconds, 60 fps
    }

    Ok(()) //return ok
}
