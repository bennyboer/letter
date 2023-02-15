pub use bounds::Bounds;
pub use constraints::LayoutConstraints;
pub use id::ElementId;
pub use layout::DocumentLayout;
pub use page::Page;
pub use position::Position;
pub use size::Size;

use crate::element::content::LayoutElementContent;

mod bounds;
mod constraints;
pub mod content;
mod id;
mod layout;
mod page;
mod position;
mod size;

#[derive(Debug)]
pub struct LayoutElement {
    id: ElementId,
    bounds: Bounds,
    content: LayoutElementContent,
}

impl LayoutElement {
    pub fn of(id: ElementId, bounds: Bounds, content: LayoutElementContent) -> Self {
        Self {
            id,
            bounds,
            content,
        }
    }

    pub fn new(bounds: Bounds, content: LayoutElementContent) -> Self {
        Self {
            id: ElementId::new(),
            bounds,
            content,
        }
    }

    pub fn id(&self) -> ElementId {
        self.id
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn bounds_mut(&mut self) -> &mut Bounds {
        &mut self.bounds
    }

    pub fn content(&self) -> &LayoutElementContent {
        &self.content
    }
}
