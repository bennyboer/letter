#[derive(Clone, Debug, PartialEq)]
pub enum FontFamilySource {
    Default,
    Type(FontFamilyType),
    Name(String),
    Path(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum FontFamilyType {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
}

impl FontFamilyType {
    pub fn name(&self) -> &'static str {
        match self {
            FontFamilyType::Serif => "serif",
            FontFamilyType::SansSerif => "sans-serif",
            FontFamilyType::Monospace => "monospace",
            FontFamilyType::Cursive => "cursive",
            FontFamilyType::Fantasy => "fantasy",
        }
    }
}
