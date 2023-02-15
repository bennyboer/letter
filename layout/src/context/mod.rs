use crate::element::LayoutConstraints;

pub(crate) struct LayoutContext {
    /// A stack of layout constraints.
    /// When iterating over the document structure each wrapper node may
    /// impose additional constraints on the layout of its children.
    /// For example a list may be indented and a table may have a fixed width.
    /// These constraints are pushed onto the stack and popped off when the wrapper node
    /// is finished being processed.
    constraints_stack: Vec<LayoutConstraints>,
}

impl LayoutContext {
    pub fn new() -> Self {
        Self {
            constraints_stack: Vec::new(),
        }
    }
}
