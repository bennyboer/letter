use std::collections::HashMap;

use document::structure::DocumentNode;
use document::style::{FontFamilySource, NodeName, Style};
use document::Document;
use font::{FontId, FontManager, LetterFont};

pub(crate) use crate::context::insets::Insets;
pub(crate) use crate::context::page_sizing::{OneSizeFitsAllPageSizing, PageSizing};
pub(crate) use crate::context::style::LayoutStyle;
use crate::element::{
    Bounds, DocumentLayout, ElementId, LayoutConstraints, LayoutElement, Page, Position, Size,
};
use crate::result::LayoutResult;

mod insets;
mod page_sizing;
mod style;

pub(crate) struct LayoutContext<'a> {
    _last_pass_layout: Option<DocumentLayout<'a>>,

    page_sizing: Box<dyn PageSizing>,

    /// A stack of layout styles.
    /// When iterating over the document structure each node may
    /// impose additional styles and constraints on the layout of its children.
    /// For example a list may be indented and a table may have a fixed width.
    /// These constraints are pushed onto the stack and popped off when the node
    /// is finished being processed.
    style_stack: Vec<LayoutStyle>,

    /// Current bounds in which to layout elements.
    /// This will be modified by each `LayoutRule` to reduce the available space.
    /// Margin and padding are already subtracted from the bounds.
    bounds: Bounds,

    /// Ordered pages currently being laid out.
    pages: Vec<Page>,

    /// All elements that have been laid out.
    element_lookup: HashMap<ElementId, LayoutElement>,

    font_manager: FontManager<'a>,
}

impl<'a> LayoutContext<'a> {
    pub fn new(
        last_pass_layout: Option<DocumentLayout<'a>>,
        page_sizing: Box<dyn PageSizing>,
    ) -> Self {
        let mut result = Self {
            _last_pass_layout: last_pass_layout,
            page_sizing,
            style_stack: Vec::new(),
            bounds: Bounds::empty(),
            pages: Vec::new(),
            element_lookup: HashMap::new(),
            font_manager: FontManager::new(),
        };

        result.push_page();

        result
    }

    pub(crate) fn to_layout(self) -> DocumentLayout<'a> {
        DocumentLayout::new(self.pages, self.element_lookup, self.font_manager)
    }

    pub(crate) fn is_stable(&self) -> bool {
        return true; // TODO Flag the layout as unstable if we have elements of unknown size
    }

    pub(crate) fn push_style(&mut self, style: LayoutStyle) {
        self.apply_style_to_bounds(&style);
        self.style_stack.push(style);
    }

    pub(crate) fn pop_style(&mut self) {
        let style = self.style_stack.pop();
        if let Some(style) = style {
            self.remove_style_from_bounds(&style);
        }
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

        self.reset_bounds();
    }

    pub(crate) fn register_element(&mut self, element: LayoutElement) {
        self.current_page().add_element(element.id());
        self.element_lookup.insert(element.id(), element);
    }

    pub(crate) fn bounds(&self) -> Bounds {
        self.bounds
    }

    pub(crate) fn set_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
    }

    /// Modify the current context to break out of the current constraints.
    /// This is used when a layout element is too large to fit on the current page (or more
    /// specifically, the current layout constraints).
    /// This may push a new page if there is not enough space on the current page.
    pub(crate) fn choose_next_bounds(&mut self) -> Bounds {
        // TODO Implement LayoutConstraintsRule to determine what to do when we break out of constraints
        // TODO Currently we simply push a new page, but we could also have a multi-column layout where we simply break to the next column (set in styles with the `layout` property)
        self.push_page();
        self.bounds
    }

    pub(crate) fn find_font(&mut self, font_family: &FontFamilySource) -> Option<FontId> {
        match font_family {
            FontFamilySource::Default => Some(self.font_manager.default_font_id()),
            FontFamilySource::Name(name) => self.font_manager.find_by_name(name.as_str()),
            FontFamilySource::Path(path) => self.font_manager.find_by_path(path.as_str()),
        }
    }

    pub(crate) fn get_font_mut(&mut self, id: &FontId) -> &mut LetterFont<'a> {
        self.font_manager.get_font_mut(id).unwrap()
    }

    pub(crate) fn push_node_styles(
        &mut self,
        node: &DocumentNode,
        document: &Document,
    ) -> LayoutResult<()> {
        let node_name: Option<NodeName> = node.name().map(|name| name.into());
        let current_style = self.current_style().clone();
        if let Some(node_name) = node_name {
            let class_name = node.class_name();
            let styles = document.styles.resolve(&node_name, class_name);
            let layout_style = self.apply_to_layout_style(current_style, &styles);

            self.push_style(layout_style);
        }

        Ok(())
    }

    pub(crate) fn pop_node_styles(&mut self, node: &DocumentNode) -> LayoutResult<()> {
        if node.name().is_some() {
            self.pop_style();
        }

        Ok(())
    }

    fn apply_to_layout_style(
        &self,
        mut layout_style: LayoutStyle,
        styles: &Vec<&Style>,
    ) -> LayoutStyle {
        // Size, margin and padding are not inherited
        layout_style.set_size(Size::max());
        layout_style.set_margin(Insets::zero());
        layout_style.set_padding(Insets::zero());

        for style in styles {
            match style {
                Style::Width(distance) => {
                    layout_style.set_size(layout_style.size().with_width(*distance))
                }
                Style::Height(distance) => {
                    layout_style.set_size(layout_style.size().with_height(*distance))
                }
                Style::MarginTop(distance) => {
                    layout_style.set_margin(layout_style.margin().with_top(*distance))
                }
                Style::MarginRight(distance) => {
                    layout_style.set_margin(layout_style.margin().with_right(*distance))
                }
                Style::MarginBottom(distance) => {
                    layout_style.set_margin(layout_style.margin().with_bottom(*distance))
                }
                Style::MarginLeft(distance) => {
                    layout_style.set_margin(layout_style.margin().with_left(*distance))
                }
                Style::PaddingTop(distance) => {
                    layout_style.set_padding(layout_style.padding().with_top(*distance))
                }
                Style::PaddingRight(distance) => {
                    layout_style.set_padding(layout_style.padding().with_right(*distance))
                }
                Style::PaddingBottom(distance) => {
                    layout_style.set_padding(layout_style.padding().with_bottom(*distance))
                }
                Style::PaddingLeft(distance) => {
                    layout_style.set_padding(layout_style.padding().with_left(*distance))
                }
                Style::FontSize(distance) => layout_style.set_font_size(*distance),
                Style::FontFamily(font_family) => layout_style.set_font_family(font_family.clone()),
                Style::FontVariationSettings(settings) => {
                    layout_style.set_font_variation_settings(settings.clone())
                }
            };
        }

        layout_style
    }

    fn current_page(&mut self) -> &mut Page {
        self.pages.last_mut().unwrap()
    }

    fn get_page_constraints(&self, page_number: usize) -> LayoutConstraints {
        self.page_sizing.get_page_constraints(page_number)
    }

    fn apply_style_to_bounds(&mut self, style: &LayoutStyle) {
        let margin = style.margin();
        let margin_top = margin.top();
        let margin_left = margin.left();
        let margin_right = margin.right();
        let margin_bottom = margin.bottom();

        let padding = style.padding();
        let padding_top = padding.top();
        let padding_left = padding.left();
        let padding_right = padding.right();
        let padding_bottom = padding.bottom();

        let current_size = self.bounds.size();

        let new_origin = Position::relative_to(
            self.bounds.position(),
            margin_left + padding_left,
            margin_top + padding_top,
        );
        let new_size = self
            .bounds
            .size()
            .with_width(
                current_size.width - margin_left - margin_right - padding_left - padding_right,
            )
            .with_height(
                current_size.height - margin_top - margin_bottom - padding_top - padding_bottom,
            );
        self.bounds = Bounds::new(new_origin, new_size);

        if self.bounds.size().is_negative() {
            self.choose_next_bounds();
        }
    }

    fn remove_style_from_bounds(&mut self, style: &LayoutStyle) {
        let margin = style.margin();
        let margin_left = margin.left();
        let margin_right = margin.right();
        let margin_bottom = margin.bottom();

        let padding = style.padding();
        let padding_left = padding.left();
        let padding_right = padding.right();
        let padding_bottom = padding.bottom();

        let current_size = self.bounds.size();

        let new_origin = Position::relative_to(
            self.bounds.position(),
            -(margin_left + padding_left),
            margin_bottom + padding_bottom,
        );
        let new_size = self
            .bounds
            .size()
            .with_width(
                current_size.width + margin_left + margin_right + padding_left + padding_right,
            )
            .with_height(current_size.height - margin_bottom - padding_bottom);
        self.bounds = Bounds::new(new_origin, new_size);
    }

    fn reset_bounds(&mut self) {
        let page_constraints = self.current_page().constraints();
        self.bounds = Bounds::new(Position::zero(), page_constraints.size());

        let styles = self.style_stack.clone();
        for style in styles {
            self.apply_style_to_bounds(&style);
        }
    }
}
