use crate::ecs::system::System;
use crate::ecs::entity_manager::EntityManager;
use std::collections::{HashMap, HashSet};
use std::any::TypeId;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SystemManager {
    systems: Vec<Rc<RefCell<dyn System>>>,
    entity_system_registry: HashMap<TypeId, HashSet<u32>>,
}

impl SystemManager {
    pub fn new() -> Self {
        SystemManager {
            systems: Vec::new(),
            entity_system_registry: HashMap::new(),
        }
    }

    pub fn add_system(&mut self, system: Rc<RefCell<dyn System>>) {
        self.systems.push(system);
        self.systems.sort_by_key(|s| s.borrow().priority());
    }

    pub fn register_entity_to_system<T: System + 'static>(&mut self, entity_id: u32) {
        let system_type_id = TypeId::of::<T>();
        self.entity_system_registry
            .entry(system_type_id)
            .or_insert_with(HashSet::new)
            .insert(entity_id);
    }

    pub fn update_systems(&mut self, entity_manager: &mut EntityManager) {
        for system in &self.systems {
            let system_type_id = system.borrow().type_id();
            if let Some(entity_ids) = self.entity_system_registry.get(&system_type_id) {
                for &entity_id in entity_ids {
                    system.borrow_mut().update(entity_manager, entity_id);
                }
            }
        }
    }
}