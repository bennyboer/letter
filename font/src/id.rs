pub type FontId = u64;

pub(crate) struct FontIdGenerator {
    next_id: FontId,
}

impl FontIdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn next(&mut self) -> FontId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
