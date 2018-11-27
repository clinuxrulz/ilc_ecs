use crate::ecs::IsComponent;
use std::any::Any;

pub struct RawComponent {
    type_name: String,
    value: Box<Any>
}

impl RawComponent {
    pub fn from<C:IsComponent+'static>(component: C) -> RawComponent {
        RawComponent {
            type_name: C::get_component_type().type_name().clone(),
            value: Box::new(component)
        }
    }
}
