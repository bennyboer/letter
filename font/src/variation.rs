pub type FontVariationId = u64;

pub(crate) struct FontVariationIdGenerator {
    next_id: FontVariationId,
}

impl FontVariationIdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn next(&mut self) -> FontVariationId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LetterFontVariation {
    name: String,
    value: i32,
}

impl LetterFontVariation {
    pub fn new(name: String, value: i32) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
