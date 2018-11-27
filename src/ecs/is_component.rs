use crate::ecs::ComponentType;

pub trait IsComponent {
    fn get_component_type() -> ComponentType<Self>;
}
