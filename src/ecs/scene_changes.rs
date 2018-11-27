use crate::ecs::SceneChange;

pub struct SceneChanges {
    changes: Vec<SceneChange>
}

impl SceneChanges {
    pub fn new(changes: Vec<SceneChange>) -> SceneChanges {
        SceneChanges {
            changes
        }
    }
}
