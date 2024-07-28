//src/ecs/entity_manager.rs
//Description: This module contains systems which are responsible for updating entities and acting on their components.
//essentially it's the logic that operaties on the data in the components.

use crate::ecs::components::Position; //for position
use crate::ecs::components::RenderData; //for RenderData
use crate::ecs::components::Velocity; //for velocity
use crate::ecs::components::PlayerData; //for player data
use crate::ecs::entity_manager::{Entity, EntityManager}; //for EntityManager
use crate::input_handler::InputHandler; // for InputHandler
use crate::sdl_window_manager::SDLWindowManager; //for SDLWindowManager

pub trait System {
    //trait for system
    fn update(&mut self, entity_manager: &mut EntityManager); //function to update
}

pub struct MovementSystem; //struct for movement system

impl MovementSystem {
    pub fn update(&mut self, entity_manager: &mut EntityManager, input_handler: &InputHandler) {
        let entities_with_velocity: Vec<_> = entity_manager
            .query_entities::<Velocity>() //query entities with position
            .iter() //for each entity
            .map(|entity| entity.id) //map entity to id
            .collect(); //collect to vector

        //for each entity with velocity check that it has a positoion and update the position based on the velocity
        for entity_id in entities_with_velocity {
            //for each entity id
            if let Some(position) =
                entity_manager.get_component_mut::<Position>(&Entity { id: entity_id })
            {
                //get mutable position
                if input_handler.is_w_pressed() {
                    position.y -= 1.0;
                }
                if input_handler.is_s_pressed() {
                    position.y += 1.0;
                }
                if input_handler.is_a_pressed() {
                    position.x -= 1.0;
                }
                if input_handler.is_d_pressed() {
                    position.x += 1.0;
                }
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