use crate::ecs::ComponentType;
use crate::ecs::EntityId;
use crate::ecs::IsComponent;

pub trait SceneCtx {
    // for Undo/Redo capabilities
    fn create_entity_with_id(&mut self, entity_id: EntityId);
    
    fn create_entity(&mut self) -> EntityId;
    fn destroy_entity(&mut self, entity_id: EntityId);
    fn get_component<A>(&self, entity_id: EntityId, component_type: ComponentType<A>) -> Option<A>;
    fn set_component<A:IsComponent>(&mut self, entity_id: EntityId, component: A);
    fn children(&self, entity_id: EntityId) -> Vec<EntityId>;
    fn entities_of_type(&self, component_type_name: &str) -> Vec<EntityId>;
}
