// src/ecs/entity_manager.rs

//Description:
//This module contains the EntityManager struct which is responsible for managing entities and components.
//It's the primary way in which entities and components are created, added, and queried and interact.
//it works, it was a nightmare to implement, but it works.

use crate::ecs::components::Component;
use std::any::{Any, TypeId}; //for any and typeid
use std::collections::HashMap; //for hashmap //for component

#[derive(Clone)]
pub struct Entity {
    // Entity struct
    pub id: u32,
}

pub struct EntityManager {
    // EntityManager struct
    next_id: u32,
    entities: Vec<Entity>,
    components: HashMap<TypeId, HashMap<u32, Box<dyn Any>>>, //Box is a pointer to the heap, Any is a trait object
}

impl EntityManager {
    // EntityManager struct
    pub fn new() -> Self {
        //new function
        EntityManager {
            //EntityManager struct
            next_id: 0,
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        //create entity function
        let entity = Entity { id: self.next_id }; //create entity
        self.next_id += 1;
        self.entities.push(entity.clone());
        entity
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        //add component function
        let components = self
            .components
            .entry(TypeId::of::<T>())
            .or_insert_with(HashMap::new); //insert component, box is just a pointer to the heap
        components.insert(entity.id, Box::new(component)); //insert component
    }

    pub fn get_component<T: Component + 'static>(&self, entity: &Entity) -> Option<&T> {
        //get component function
        self.components
            .get(&TypeId::of::<T>()) //get component
            .and_then(|components| components.get(&entity.id)) //get component
            .and_then(|component| component.downcast_ref::<T>()) //downcasting is a way to convert a trait object back into a concrete type so that you can access its methods
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        //get mutable component function
        self.components
            .get_mut(&TypeId::of::<T>()) //get mutable component
            .and_then(|components| components.get_mut(&entity.id)) //get mutable component
            .and_then(|component| component.downcast_mut::<T>()) //get mutable component 
    }

    pub fn query_entities<T: Component + 'static>(&self) -> Vec<&Entity> {
        //query entities function
        if let Some(components) = self.components.get(&TypeId::of::<T>()) {
            //get components
            components
                .keys() //get keys
                .filter_map(|id| self.entities.iter().find(|entity| entity.id == *id)) //filter map
                .collect() //collect
        } else {
            //if no components
            Vec::new() //return empty vector
        }
    }
}
