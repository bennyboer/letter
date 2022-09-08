use document::structure::NodeId;

#[derive(Debug)]
pub(crate) struct TextBlock {
    pub(crate) parts: Vec<TextBlockPart>,
}

/// Part of a text block that differs from
/// the other parts in that it has another node of
/// the document structure as parent.
#[derive(Debug)]
pub(crate) struct TextBlockPart {
    pub(crate) value: TextBlockPartValue,
    pub(crate) document_structure_node: NodeId,
}

#[derive(Debug)]
pub(crate) enum TextBlockPartValue {
    Text(TextBlockPartTextValue),
    Reference, // TODO other things like inline code?, math. expressions, references, that alter typesetting
}

#[derive(Debug)]
pub(crate) struct TextBlockPartTextValue {
    pub(crate) value: String,
}
