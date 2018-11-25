pub use crate::ecs::ComponentType;

pub trait IsComponent {
    fn get_component_type(&self) -> ComponentType<Self>;
}
