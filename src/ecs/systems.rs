//src/ecs/entity_manager.rs
//Description: This module contains systems which are responsible for updating entities and acting on their components.
//essentially it's the logic that operaties on the data in the components.

use crate::ecs::components::Position; //for position
use crate::ecs::components::RenderData; //for RenderData
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
        let entities_with_position: Vec<_> = entity_manager
            .query_entities::<Position>() //query entities with position
            .iter() //for each entity
            .map(|entity| entity.id) //map entity to id
            .collect(); //collect to vector

        for entity_id in entities_with_position {
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