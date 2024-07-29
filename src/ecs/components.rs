// src/ecs/components.rs

//Description:
//This module contains components that can be attached to entities. Components are data that can be attached to entities to give them properties.
//Components are used by systems to update entities. They do not complain logic.

pub trait Component {} //trait for component

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {} //implementation of component for velocity

impl Component for Position {} //implementation of component for position

#[derive(Clone)]
pub struct RenderData {
    pub size: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Component for RenderData {} //implementation of component for render data

pub struct PlayerData{
    //empty for now
}

impl Component for PlayerData{} //implementation of component for player data

#[derive(Clone)]
pub struct CollisionData;

impl Component for CollisionData{} //implementation of component for collision data