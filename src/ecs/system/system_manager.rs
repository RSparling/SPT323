use crate::ecs::entity_manager::EntityManager;
use crate::ecs::system::System;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// SystemManager struct to manage systems and their associated entities
pub struct SystemManager {
    // Vector of systems, each wrapped in Rc and RefCell for shared ownership and interior mutability
    systems: Vec<Rc<RefCell<dyn System>>>,
    // Registry mapping system TypeIds to sets of entity IDs
    entity_system_registry: HashMap<TypeId, HashSet<u32>>,
}

impl SystemManager {
    // Constructor for SystemManager, initializes empty systems vector and registry
    pub fn new() -> Self {
        SystemManager {
            systems: Vec::new(),
            entity_system_registry: HashMap::new(),
        }
    }

    // Adds a system to the manager and sorts systems by their priority
    pub fn add_system(&mut self, system: Rc<RefCell<dyn System>>) {
        self.systems.push(system);
        self.systems.sort_by_key(|s| s.borrow().priority());
    }

    // Registers an entity to a specific system type
    pub fn register_entity_to_system<T: System + 'static>(&mut self, entity_id: u32) {
        let system_type_id = TypeId::of::<T>();
        self.entity_system_registry
            .entry(system_type_id)
            .or_insert_with(HashSet::new)
            .insert(entity_id);
    }

    // Updates all systems, passing the entity manager and relevant entity IDs to each system's update method
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
