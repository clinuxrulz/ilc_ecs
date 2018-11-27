use crate::ecs::EntityId;
use crate::ecs::SceneCtx;

use std::any::Any;
use std::collections::HashMap;
use std::collections::BTreeMap;

pub struct Scene {
    entities: HashMap<EntityId,BTreeMap<String,Box<Any>>>,
    next_id: EntityId,
    free_ids: Vec<EntityId>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            entities: HashMap::new(),
            next_id: EntityId::new(0),
            free_ids: Vec::new()
        }
    }

    fn alloc_id(&mut self) -> EntityId {
        if let Some(id) = self.free_ids.pop() {
            return id;
        }
        self.inc_next_id_marker_to_free_id();
        self.next_id.clone()
    }

    fn free_id(&mut self, id: EntityId) {
        self.free_ids.push(id);
    }

    fn inc_next_id_marker_to_free_id(&mut self) {
        let mut next_id = self.next_id.clone();
        while self.entities.contains_key(&next_id) {
            next_id = next_id.next_id();
        }
        self.next_id = next_id;
    }
}

/* TODO: Finish this
impl SceneCtx for Scene {

    fn create_entity_with_id(&mut self, entity_id: EntityId) {
        self.entities.insert(entity_id, BTreeMap::new());
    }
    
    fn create_entity(&mut self) -> EntityId {

    }

    fn destroy_entity(&mut self, entity_id: EntityId);
    fn get_component<A>(&self, entity_id: EntityId, component_type: ComponentType<A>) -> Option<A>;
    fn set_component<A:IsComponent>(&mut self, entity_id: EntityId, component: A);
    fn children(&self, entity_id: EntityId) -> Vec<EntityId>;
    fn entities_of_type(&self, component_type_name: &str) -> Vec<EntityId>;
}
*/