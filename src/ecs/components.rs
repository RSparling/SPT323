// src/ecs/components.rs

//Description:
//This module contains components that can be attached to entities. Components are data that can be attached to entities to give them properties.
//Components are used by systems to update entities. They do not complain logic.

pub trait Component {} //trait for component

#[derive(Clone)] //derive clone means that the struct can be cloned and copied
pub struct Position {
    //struct for position
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Velocity {
    //struct for velocity
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {} //implementation of component for velocity

impl Component for Position {} //implementation of component for position

#[derive(Clone)] //derive clone for render data
pub struct RenderData {
    //struct for render data
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub size: f32,
}

impl Component for RenderData {} //implementation of component for render data

pub struct PlayerData{
    //empty for now
}

impl Component for PlayerData{} //implementation of component for player data