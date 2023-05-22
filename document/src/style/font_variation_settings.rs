#[derive(Clone, Debug, PartialEq)]
pub struct FontVariationSettings {
    pub variations: Vec<FontVariation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontVariation {
    pub name: String,
    pub value: i32,
}
