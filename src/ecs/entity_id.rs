use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};

pub struct EntityId(u32);

impl Clone for EntityId {
    fn clone(&self) -> Self {
        let EntityId(id) = self;
        EntityId(id.clone())
    }
}

impl Copy for EntityId {}

impl Hash for EntityId {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let EntityId(id) = self;
        id.hash(hasher);
    }
}

impl PartialEq for EntityId {
    fn eq(&self, rhs: &Self) -> bool {
        let EntityId(lhs_id) = self;
        let EntityId(rhs_id) = rhs;
        return lhs_id == rhs_id;
    }
}

impl Eq for EntityId {}

impl EntityId {
    pub fn new(id: u32) -> EntityId {
        EntityId(id)
    }

    pub fn next_id(&self) -> EntityId {
        let EntityId(id) = self;
        EntityId(id + 1)
    }
}
