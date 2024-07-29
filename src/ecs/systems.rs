//src/ecs/entity_manager.rs
//Description: This module contains systems which are responsible for updating entities and acting on their components.
//essentially it's the logic that operaties on the data in the components.

use crate::ecs::components::Position; //for position
use crate::ecs::components::RenderData; //for RenderData
use crate::ecs::components::Velocity; //for velocity
use crate::ecs::components::PlayerData; //for player data
use crate::ecs::components::CollisionData; //for collision data
use crate::ecs::entity_manager::{Entity, EntityManager}; //for EntityManager
use crate::input_handler::InputHandler; // for InputHandler
use crate::sdl_window_manager::SDLWindowManager; //for SDLWindowManager

pub trait System {
    fn update(&mut self, entity_manager: &mut EntityManager); // function to update
}

pub struct MovementSystem; //struct for movement system

impl System for MovementSystem {
    fn update(&mut self, entity_manager: &mut EntityManager) {
        // Collect all entities with both Position and Velocity components
        let entities_to_update: Vec<(u32, Velocity)> = entity_manager
            .query_entities::<Velocity>() // query entities with Velocity
            .iter()
            .filter_map(|entity| {
                let entity_id = entity.id;
                if let Some(velocity) = entity_manager.get_component::<Velocity>(entity) {
                    if entity_manager.get_component::<Position>(entity).is_some() {
                        return Some((entity_id, velocity.clone()));
                    }
                }
                None
            })
            .collect();

        // Update positions based on velocities
        for (entity_id, velocity) in entities_to_update {
            if let Some(position) = entity_manager.get_component_mut::<Position>(&Entity { id: entity_id }) {
                position.x += velocity.x;
                position.y += velocity.y;
            }
        }
    }
}

pub struct RenderSystem<'a> {
    //struct for render system
    pub window_manager: &'a mut SDLWindowManager, //window manager
}

impl<'a> System for RenderSystem<'a> {
    //implementation of system for render system
    fn update(&mut self, entity_manager: &mut EntityManager) {
        //function to update
        self.window_manager.clear(); //clear window

        for entity in entity_manager.query_entities::<RenderData>() {
            //for each entity with render data
            if let Some(render_data) = entity_manager.get_component::<RenderData>(entity) {
                //get render data
                if let Some(position) = entity_manager.get_component::<Position>(entity) {
                    //get position
                    self.window_manager.draw_square(
                        //draw square
                        position.x as i32,
                        position.y as i32,
                        render_data.size as u32,
                        (render_data.r * 255.0) as u8,
                        (render_data.g * 255.0) as u8,
                        (render_data.b * 255.0) as u8,
                    );
                }
            }
        }

        self.window_manager.present(); //present window
    }
}

pub struct PlayerController {}

impl PlayerController  {//to be placed on player entity, sets the player's velocity based on input. If no input, sets velocity to 0.
    pub fn update(&mut self, entity_manager: &mut EntityManager, input_handler: &InputHandler) {
        //get entities with player data
        let player_entities: Vec<_> = entity_manager
            .query_entities::<PlayerData>()
            .iter()
            .map(|entity| entity.id)
            .collect();

            //if player_entities is empty then print "no player entities"
        if player_entities.is_empty() {
            println!("No player entities");
        }
        //for each player entity, set velocity based on input
        //if no input, set velocity to 0
        for entity_id in player_entities {
            if let Some(velocity) = entity_manager.get_component_mut::<Velocity>(&Entity { id: entity_id }) {
                velocity.x = 0.0;
                velocity.y = 0.0;
            }
            else{
                println!("No velocity component for player entity");
            }
            //check input
            if let Some(velocity) = entity_manager.get_component_mut::<Velocity>(&Entity { id: entity_id }) {
                if input_handler.is_w_pressed() {
                    velocity.y -= 1.0;
                }
                if input_handler.is_s_pressed() {
                    velocity.y += 1.0;
                }
                if input_handler.is_a_pressed() {
                    velocity.x -= 1.0;
                }
                if input_handler.is_d_pressed() {
                    velocity.x += 1.0;
                }
            }
            else{
                println!("No velocity component for player entity");
            }
        }
    }
}

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&mut self, entity_manager: &mut EntityManager) {
        // Collect the necessary data immutably first
        let mut collision_entities: Vec<(u32, f32, f32, f32)> = Vec::new();

        for entity in entity_manager.query_entities::<CollisionData>() {
            if let Some(position) = entity_manager.get_component::<Position>(entity) {
                if let Some(render_data) = entity_manager.get_component::<RenderData>(entity) {
                    collision_entities.push((entity.id, position.x, position.y, render_data.size));
                }
            }
        }

        // Perform the mutable updates in a separate loop
        for (entity_id, pos_x, pos_y, size) in collision_entities {
            if let Some(velocity) = entity_manager.get_component_mut::<Velocity>(&Entity { id: entity_id }) {
                if pos_x < 0.0 {
                    velocity.x = 1.0;
                }
                if pos_x + size > 800.0 {
                    velocity.x = -1.0;
                }
                if pos_y < 0.0 {
                    velocity.y = 1.0;
                }
                if pos_y + size > 600.0 {
                    velocity.y = -1.0;
                }
            }
        }
    }
}
