mod author;
mod encoding;
mod language;
mod variables;
mod version;

pub use author::Author;
pub use encoding::DocumentEncoding;
pub use language::DocumentLanguage;
pub use variables::DocumentVariables;
pub use version::DocumentVersion;

#[derive(Default, Debug)]
pub struct DocumentMetaData {
    pub encoding: DocumentEncoding,
    pub language: DocumentLanguage,
    pub authors: Vec<Author>,
    pub version: DocumentVersion,
    pub variables: DocumentVariables,
}
