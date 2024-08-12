// src/ecs/components.rs

//Description:
//This module contains components that can be attached to entities. Components are data that can be attached to entities to give them properties.
//Components are used by systems to update entities. They do not contain logic.

pub mod transform_data;
pub mod collision_data;
pub mod render_data;
pub mod player_data;

pub trait Component {} //trait for component