use unit::Distance;

use crate::element::{DocumentLayout, LayoutConstraints};

pub(crate) struct LayoutContext {
    _last_pass_layout: Option<DocumentLayout>,

    /// A stack of layout constraints.
    /// When iterating over the document structure each wrapper node may
    /// impose additional constraints on the layout of its children.
    /// For example a list may be indented and a table may have a fixed width.
    /// These constraints are pushed onto the stack and popped off when the wrapper node
    /// is finished being processed.
    constraints_stack: Vec<LayoutConstraints>,

    /// Current offset from the top of the page.
    _offset: Distance,
}

impl LayoutContext {
    pub fn new(last_pass_layout: Option<DocumentLayout>) -> Self {
        Self {
            _last_pass_layout: last_pass_layout,
            constraints_stack: Vec::new(),
            _offset: Distance::zero(),
        }
    }

    pub(crate) fn push_layout_constraints(&mut self, layout_constraints: LayoutConstraints) {
        self.constraints_stack.push(layout_constraints);
    }
}
