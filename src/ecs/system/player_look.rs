// src/ecs/system/player_look.rs
// Description:
// This module contains the player look system, responsible for casting rays to detect walls in the world.

use sdl2::pixels::Color;
use sdl2::sys::{ldiv_t, XOC};

use crate::ecs::component::player_data::PlayerData;
use crate::ecs::component::transform_data::Transform;
use crate::ecs::component::world_data::WorldData;
use crate::ecs::entity_manager::EntityManager;
use crate::sdl_window_manager::SDLWindowManager;
use std::any::Any;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::f32::INFINITY;
use std::rc::Rc;

use super::System;
pub struct PlayerLook {
    pub window_manager: Rc<RefCell<SDLWindowManager>>,
}

impl PlayerLook {
    /// Casts rays from the player's position and handles horizontal and vertical intersections.
    fn cast_rays(&mut self, player: &Transform, world_data: &WorldData) {
        let degree = (120.0 / 60.0) * 0.0174533; // Angle increment per ray
        let mut ray_angle = player.position.rotation() - 60.0 * 0.0174533; // Start ray_angle 60 degrees left
    
        let window_width = {
            let mut window_manager = self.window_manager.borrow_mut();
            window_manager.get_window_size().0
        };
        
        let ray_width = (window_width / 60) as i32; // Width of each ray slice
        
        for r in 0..60 {
            let hit_vert: (f32, f32) = self.cast_vertical(player, world_data, ray_angle);
            let hit_horz: (f32, f32) = self.cast_horizontal(player, world_data, ray_angle);
    
            let dist_horz = self.calculate_distance(
                player.position.x(),
                player.position.y(),
                hit_horz.0,
                hit_horz.1,
            );
            let dist_vert = self.calculate_distance(
                player.position.x(),
                player.position.y(),
                hit_vert.0,
                hit_vert.1,
            );
    
            let dist = if dist_horz < dist_vert {
                dist_horz
            } else {
                dist_vert
            };
    
            let mut color: Color = if dist_horz < dist_vert {
                Color::RGB(255, 0, 0) // Color for horizontal walls
            } else {
                Color::RGB(0, 255, 0) // Color for vertical walls
            };
    
            let line_height: i32 = ((world_data.cell_size as f32 / dist) * 100.0) as i32;
            let max_line_height = world_data.cell_size as i32 * 10;
            let line_height = if line_height > max_line_height {
                max_line_height
            } else {
                line_height
            };
            let line_offset: i32;
            {
                let mut window_manager = self.window_manager.borrow_mut();
                line_offset = (window_manager.get_window_size().1 as f32 / 2.0) as i32;
            }
            let top: i32 = line_offset - (line_height / 2);
            let bottom: i32 = line_offset + (line_height / 2);
    
            let rect = (r * ray_width, top, ray_width, line_height);
    
            // Draw the wall slice as a filled rectangle
            {
                let mut window_manager = self.window_manager.borrow_mut();
                window_manager.draw_rect(rect.0 as i32, rect.1 as i32, rect.2 as u32, rect.3 as u32, color.r, color.g, color.b);
            }
    
            // Increment ray angle for next ray
            ray_angle += degree;
            if ray_angle < 0.0 {
                ray_angle += 2.0 * PI;
            }
            if ray_angle > 2.0 * PI {
                ray_angle -= 2.0 * PI;
            }
        }
    }
    
    
    fn calculate_distance(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    }
    fn cast_vertical(
        &mut self,
        player: &Transform,
        world_data: &WorldData,
        angle: f32,
    ) -> (f32, f32) {
        let mut ray_x: f32 = 0.0;
        let mut ray_y: f32 = 0.0;
        let mut dof: i32 = 0;
        let mut map_x: i32 = 0;
        let mut map_y: i32 = 0;
        let mut xo: f32 = 0.0;
        let mut yo: f32 = 0.0;
        let mut hit: (f32, f32) = (0.0, 0.0);
        let mut ray_angle = angle; //rotation in radians
        let pc: (f32, f32) = (player.position.x(), player.position.y());
        let tan = ray_angle.tan();

        if ray_angle.cos() > 0.0001 {
            //looking left
            //round player position down to the nearest cell
            //rx=(((int)px>>6)<<6)+64;      ry=(px-rx)*Tan+py; xo= 64; yo=-xo*Tan;
            ray_x = (((pc.0 / world_data.cell_size as f32) as i32) * world_data.cell_size as i32)
                as f32
                + world_data.cell_size as f32;
            ray_y = (pc.0 - ray_x) * tan + pc.1;
            xo = world_data.cell_size as f32;
            yo = -xo * tan;
        } else if ray_angle.cos() < -(0.0001) {
            //looking right
            //round player position up to the nearest cell
            //rx=(((int)px>>6)<<6) -0.0001; ry=(px-rx)*Tan+py; xo=-64; yo=-xo*Tan;
            ray_x = (((pc.0 / world_data.cell_size as f32) as i32) * world_data.cell_size as i32)
                as f32
                - 0.0001;
            ray_y = (pc.0 - ray_x) * tan + pc.1;
            xo = -(world_data.cell_size as f32);
            yo = -xo * tan;
        } else {
            dof = 20;
            ray_x = pc.0;
            ray_y = pc.1;
        }

        while dof < 20 {
            //map_x from rounding ray x down to the nearest cell
            map_x = ray_x as i32 / world_data.cell_size as i32;
            //map_y rounding ray y down to the nearest cell
            map_y = ray_y as i32 / world_data.cell_size as i32;
            if world_data.is_wall(map_x, map_y) {
                hit = (ray_x, ray_y);
                break;
            } else {
                ray_x += xo;
                ray_y += yo;
                dof += 1;
            }
        }
        return hit;
    }

    fn dr() -> f32 {
        return 0.0174533;
    }
    fn cast_horizontal(
        &mut self,
        player: &Transform,
        world_data: &WorldData,
        angle: f32,
    ) -> (f32, f32) {
        let mut ray_x: f32 = 0.0;
        let mut ray_y: f32 = 0.0;
        let mut dof: i32 = 0;
        let mut map_x: i32 = 0;
        let mut map_y: i32 = 0;
        let mut xo: f32 = 0.0;
        let mut yo: f32 = 0.0;
        let mut hit: (f32, f32) = (0.0, 0.0);
        let mut ray_angle = angle; //rotation in radians
        let pc: (f32, f32) = (player.position.x(), player.position.y());
        let tan = ray_angle.tan();
        let atan = 1.0 / tan;
        if ray_angle.sin() > 0.001 {
            //looking up
            ray_y = (((pc.1 / world_data.cell_size as f32) as i32) * world_data.cell_size as i32)
                as f32
                - 0.0001;
            ray_x = (pc.1 - ray_y) * atan + pc.0;
            yo = -(world_data.cell_size as f32);
            xo = -yo * atan;
        } else if ray_angle.sin() < -(0.001) {
            //looking down
            ray_y = ((((pc.0 / world_data.cell_size as f32) as i32) * world_data.cell_size as i32)
                + world_data.cell_size as i32) as f32;
            ray_x = (pc.1 - ray_y) * atan + pc.0;
            yo = world_data.cell_size as f32;
            xo = -yo * atan;
        } else {
            dof = 20;
            ray_x = pc.0;
            ray_y = pc.1;
        }

        while dof < 20 {
            map_x = ray_x as i32 / world_data.cell_size as i32;
            map_y = ray_y as i32 / world_data.cell_size as i32;
            if world_data.is_wall(map_x, map_y) {
                hit = (ray_x, ray_y);
                break;
            } else {
                ray_x += xo;
                ray_y += yo;
                dof += 1;
            }
        }
        return hit;
    }
    
    /// Draws the ray from the player's position to the hit position or maximum length.
    fn draw_ray(
        &mut self,
        player_pos: (f32, f32),
        hit_pos: (f32, f32),
        color: sdl2::pixels::Color,
    ) {
        let mut window_manager = self.window_manager.borrow_mut();
        window_manager.draw_line(
            player_pos.0 as i32,
            player_pos.1 as i32,
            hit_pos.0 as i32,
            hit_pos.1 as i32,
            color.r,
            color.g,
            color.b,
        );
    }
}

impl System for PlayerLook {
    fn update(&mut self, entity_manager: &mut EntityManager, _entity_id: u32) {
        let player_entities = entity_manager.query_entities::<PlayerData>();

        // Ensure there's exactly one player entity
        if player_entities.len() != 1 {
            panic!("There must be exactly one player entity with PlayerData.");
        }

        let player_entity_id = player_entities[0];

        // Get the player's transform
        let transform = entity_manager
            .get_component::<Transform>(&player_entity_id)
            .expect("Failed to get Transform component");

        // Get the world data
        let world_entities = entity_manager.query_entities::<WorldData>();
        if world_entities.len() != 1 {
            panic!("There must be exactly one world entity with WorldData.");
        }

        let world_data = entity_manager
            .get_component::<WorldData>(&world_entities[0])
            .expect("Failed to get WorldData component");

        // Cast rays from the player's position
        self.cast_rays(transform, world_data);
    }

    fn priority(&self) -> u32 {
        150
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
