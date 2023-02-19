use crate::meta_data::DocumentMetaData;
use crate::structure::DocumentStructure;
use crate::style::DocumentStyles;

pub mod meta_data;
pub mod structure;
pub mod style;

pub struct Document {
    pub meta_data: DocumentMetaData,
    pub structure: DocumentStructure,
    pub styles: DocumentStyles,
}
