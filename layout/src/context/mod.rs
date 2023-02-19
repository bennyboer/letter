use std::collections::HashMap;

pub(crate) use crate::context::page_sizing::{OneSizeFitsAllPageSizing, PageSizing};
pub(crate) use crate::context::style::LayoutStyle;
use crate::element::{DocumentLayout, ElementId, LayoutConstraints, LayoutElement, Page, Position};

mod insets;
mod page_sizing;
mod style;

pub(crate) struct LayoutContext {
    _last_pass_layout: Option<DocumentLayout>,

    page_sizing: Box<dyn PageSizing>,

    /// A stack of layout styles.
    /// When iterating over the document structure each node may
    /// impose additional styles and constraints on the layout of its children.
    /// For example a list may be indented and a table may have a fixed width.
    /// These constraints are pushed onto the stack and popped off when the node
    /// is finished being processed.
    style_stack: Vec<LayoutStyle>,

    /// Current offset on the page.
    offset: Position,

    /// Ordered pages currently being laid out.
    pages: Vec<Page>,

    /// All elements that have been laid out.
    element_lookup: HashMap<ElementId, LayoutElement>,
}

impl LayoutContext {
    pub fn new(last_pass_layout: Option<DocumentLayout>, page_sizing: Box<dyn PageSizing>) -> Self {
        Self {
            _last_pass_layout: last_pass_layout,
            page_sizing,
            style_stack: Vec::new(),
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

    pub(crate) fn push_style(&mut self, style: LayoutStyle) {
        self.style_stack.push(style);
    }

    pub(crate) fn pop_style(&mut self) {
        self.style_stack.pop();
    }

    pub(crate) fn current_style(&mut self) -> &LayoutStyle {
        if self.style_stack.is_empty() {
            self.style_stack.push(LayoutStyle::new());
        }

        self.style_stack.last().unwrap()
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

    /// Modify the current context to break out of the current constraints.
    /// This is used when a layout element is too large to fit on the current page (or more
    /// specifically, the current layout constraints).
    /// This may push a new page if there is not enough space on the current page.
    pub(crate) fn _break_out_of_constraints(&mut self) {
        // TODO Implement LayoutConstraintsRule to determine what to do when we break out of constraints
        // TODO Currently we simply push a new page, but we could also have a multi-column layout where we simply break to the next column (set in styles with the `layout` property)
        todo!()
    }

    fn current_page(&mut self) -> &mut Page {
        self.pages.last_mut().unwrap()
    }

    fn get_page_constraints(&self, page_number: usize) -> LayoutConstraints {
        self.page_sizing.get_page_constraints(page_number)
    }
}
