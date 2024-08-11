//ecs::system::RenderSystem.rs
//Description:
//This module contains the render system which is responsible for rendering entities to the screen.
use crate::ecs::entity_manager::EntityManager; //use entity manager and entity
use crate::SDLWindowManager; //use window manager
use crate::ecs::system::System; //use system trait

use crate::ecs::component::{position_data, render_data}; //use Position and RenderData components

pub struct RenderSystem<'a> {
    //struct for render system
    pub window_manager: &'a mut SDLWindowManager, //window manager
}

impl<'a> System for RenderSystem<'a> {
    //implementation of system for render system
    fn update(&mut self, entity_manager: &mut EntityManager) {
        //function to update
        self.window_manager.clear(); //clear window

        for entity in entity_manager.query_entities::<render_data::RenderData>() {
            //for each entity with render data
            if let Some(render_data) = entity_manager.get_component::<render_data::RenderData>(entity) {
                //get render data
                if let Some(position) = entity_manager.get_component::<position_data::Position>(entity) {
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