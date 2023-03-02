#[derive(Clone, Debug)]
pub struct FontVariationSettings {
    pub variations: Vec<FontVariation>,
}

#[derive(Clone, Debug)]
pub struct FontVariation {
    pub name: String,
    pub value: i32,
}
