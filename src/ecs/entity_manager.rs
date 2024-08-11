use crate::ecs::component::Component;
use crate::ecs::system::System;
use crate::ecs::system::system_manager::SystemManager;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Entity {
    pub id: u32,
}

pub struct EntityManager {
    next_id: u32,
    entities: Vec<Entity>,
    components: HashMap<TypeId, HashMap<u32, Box<dyn Any>>>,
    system_manager: Rc<RefCell<SystemManager>>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_id: 0,
            entities: Vec::new(),
            components: HashMap::new(),
            system_manager: Rc::new(RefCell::new(SystemManager::new())),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity { id: self.next_id };
        self.next_id += 1;
        self.entities.push(entity.clone());
        entity
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: &Entity, component: T) {
        let components = self
            .components
            .entry(TypeId::of::<T>())
            .or_insert_with(HashMap::new);
        components.insert(entity.id, Box::new(component));
    }

    pub fn get_component<T: Component + 'static>(&self, entity: &Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|components| components.get(&entity.id))
            .and_then(|component| component.downcast_ref::<T>())
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|components| components.get_mut(&entity.id))
            .and_then(|component| component.downcast_mut::<T>())
    }

    pub fn query_entities<T: Component + 'static>(&self) -> Vec<&Entity> {
        if let Some(components) = self.components.get(&TypeId::of::<T>()) {
            components
                .keys()
                .filter_map(|id| self.entities.iter().find(|entity| entity.id == *id))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn add_system(&mut self, system: Rc<RefCell<dyn System>>) {
        self.system_manager.borrow_mut().add_system(system);
    }

    pub fn register_entity_to_system<T: System + 'static>(&mut self, entity: &Entity) {
        self.system_manager.borrow_mut().register_entity_to_system::<T>(entity.id);
    }

    pub fn update(&mut self) {
        let system_manager = Rc::clone(&self.system_manager);
        
        // Borrow the system manager mutably
        system_manager.borrow_mut().update_systems(self);
    }
}
