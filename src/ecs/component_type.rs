use std::marker::PhantomData;

pub struct ComponentType<A:?Sized> {
    type_name: String,
    phantom: PhantomData<A>
}

impl<A> ComponentType<A> {
    pub fn new(type_name: String) -> ComponentType<A> {
        ComponentType {
            type_name,
            phantom: PhantomData
        }
    }

    pub fn type_name(&self) -> &String {
        &self.type_name
    }
}