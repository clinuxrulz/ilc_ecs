use crate::ecs::EntityId;
use crate::ecs::RawComponent;

pub enum SceneChange {
    EntityCreated(EntityId),
    EntityDestroyed(EntityId),
    ComponentSet(EntityId, RawComponent),
    ComponentUnset(EntityId, String)
}
