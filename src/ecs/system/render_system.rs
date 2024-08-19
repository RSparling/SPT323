/// src/ecs/system/render_system.rs
/// Description:
/// This module contains the render system which is responsible for rendering entities in the world.
use sdl2::render;
use crate::ecs::entity_manager::EntityManager;
use crate::sdl_window_manager::SDLWindowManager;
use crate::ecs::component::{transform_data::Transform, render_data::RenderData};
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

use super::System;

pub struct RenderSystem {
    pub window_manager: Rc<RefCell<SDLWindowManager>>,
}

impl RenderSystem {
    /// Draws a single entity using its transform and render data
    fn draw_entity(&self, window_manager: &mut SDLWindowManager, transform: &Transform, render_data: &RenderData) {
        window_manager.draw_filled_rect(
            transform.position.x() as i32,
            transform.position.y() as i32,
            render_data.size as u32,
            render_data.size as u32,
            (render_data.r * 255.0) as u8,
            (render_data.g * 255.0) as u8,
            (render_data.b * 255.0) as u8,
        );
    }
}

impl System for RenderSystem {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        let mut window_manager = self.window_manager.borrow_mut();

        window_manager.clear();

        for entity in entity_manager.query_entities::<RenderData>() {
            if let Some(render_data) = entity_manager.get_component::<RenderData>(entity) {
                if let Some(transform) = entity_manager.get_component::<Transform>(entity) {
                    // Delegate the drawing task to a separate method
                    self.draw_entity(&mut window_manager, transform, render_data);
                }
            }
        }
    }

    fn priority(&self) -> u32 {
        100
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
