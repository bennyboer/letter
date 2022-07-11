pub mod meta_data;
pub mod structure;

use crate::meta_data::DocumentMetaData;
use crate::structure::DocumentStructure;

pub struct Document {
    pub meta_data: DocumentMetaData,
    pub structure: DocumentStructure,
    // TODO Style model with which you can query the styling of each node in the document structure
}
