pub use bounds::Bounds;
pub use id::ElementId;
pub use layout::DocumentLayout;
pub use page::Page;
pub use page_constraints::PageConstraints;
pub use position::Position;
pub use size::Size;

mod bounds;
mod id;
mod layout;
mod page;
mod page_constraints;
mod position;
mod size;

#[derive(Debug)]
pub struct TypesetElement {
    id: ElementId,
    bounds: Bounds,
    content: TypesetElementContent,
}

impl TypesetElement {
    pub fn of(id: ElementId, bounds: Bounds, content: TypesetElementContent) -> Self {
        Self {
            id,
            bounds,
            content,
        }
    }

    pub fn new(bounds: Bounds, content: TypesetElementContent) -> Self {
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

    pub fn content(&self) -> &TypesetElementContent {
        &self.content
    }
}

#[derive(Debug)]
pub enum TypesetElementContent {
    Page,
    Group(TypesetElementGroup),
    TextSlice(TextSliceContent),
    Image,
}

#[derive(Debug)]
pub struct TypesetElementGroup {
    pub elements: Vec<ElementId>,
}

#[derive(Debug)]
pub struct TextSliceContent {
    pub text: String,
}
