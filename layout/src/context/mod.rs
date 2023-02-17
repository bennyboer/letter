use std::collections::HashMap;

use crate::element::{DocumentLayout, ElementId, LayoutConstraints, LayoutElement, Page, Position};

pub(crate) struct LayoutContext {
    _last_pass_layout: Option<DocumentLayout>,

    /// A stack of layout constraints.
    /// When iterating over the document structure each wrapper node may
    /// impose additional constraints on the layout of its children.
    /// For example a list may be indented and a table may have a fixed width.
    /// These constraints are pushed onto the stack and popped off when the wrapper node
    /// is finished being processed.
    constraints_stack: Vec<LayoutConstraints>,

    /// Current offset on the page.
    offset: Position,

    /// Ordered pages currently being laid out.
    pages: Vec<Page>,

    /// All elements that have been laid out.
    element_lookup: HashMap<ElementId, LayoutElement>,
}

impl LayoutContext {
    pub fn new(last_pass_layout: Option<DocumentLayout>) -> Self {
        Self {
            _last_pass_layout: last_pass_layout,
            constraints_stack: Vec::new(),
            offset: Position::zero(),
            pages: Vec::new(),
            element_lookup: HashMap::new(),
        }
    }

    pub(crate) fn to_layout(self) -> DocumentLayout {
        DocumentLayout::new(self.pages, self.element_lookup)
    }

    pub(crate) fn is_stable(&self) -> bool {
        return true; // TODO Flag the layout as unstable if we have elements of unknown size
    }

    pub(crate) fn push_layout_constraints(&mut self, layout_constraints: LayoutConstraints) {
        self.constraints_stack.push(layout_constraints);
    }

    pub(crate) fn pop_layout_constraints(&mut self) {
        self.constraints_stack.pop();
    }

    pub(crate) fn push_page(&mut self) {
        let page_number = self.pages.len() + 1;
        let page_constraints = self.get_page_constraints(page_number);
        let page = Page::new(page_number, page_constraints);

        self.pages.push(page);
    }

    pub(crate) fn register_element(&mut self, element: LayoutElement) {
        self.current_page().add_element(element.id());
        self.element_lookup.insert(element.id(), element);
    }

    pub(crate) fn offset(&self) -> Position {
        self.offset
    }

    pub(crate) fn set_offset(&mut self, offset: Position) {
        self.offset = offset;
    }

    fn current_page(&mut self) -> &mut Page {
        self.pages.last_mut().unwrap()
    }

    fn get_page_constraints(&self, _number: usize) -> LayoutConstraints {
        // TODO Determine page constraints from document styles
        // TODO Currently we simply take the first known constraints
        self.constraints_stack.first().unwrap().clone()
    }
}
