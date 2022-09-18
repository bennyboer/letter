use uuid::Uuid;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ElementId {
    value: Uuid,
}

impl ElementId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }
}
