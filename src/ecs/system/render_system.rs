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

impl System for RenderSystem {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        
        let window_manager = self.window_manager.borrow_mut();

        window_manager.clear();

        for entity in entity_manager.query_entities::<RenderData>() {
            if let Some(render_data) = entity_manager.get_component::<RenderData>(entity) {
                if let Some(transform) = entity_manager.get_component::<Transform>(entity) {
                    window_manager.draw_square(
                        transform.position.pos_x as i32,
                        transform.position.pos_y as i32,
                        render_data.size as u32,
                        (render_data.r * 255.0) as u8,
                        (render_data.g * 255.0) as u8,
                        (render_data.b * 255.0) as u8,
                    );
                }
            }
        }

        window_manager.present();
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
