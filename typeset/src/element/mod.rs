pub use bounds::Bounds;
pub use id::ElementId;
pub use page::Page;
pub use position::Position;
pub use size::Size;
pub use page_constraints::PageConstraints;

mod bounds;
mod id;
mod page;
mod position;
mod size;
mod page_constraints;

#[derive(Debug)]
pub struct TypesetElement {
    id: ElementId,
    bounds: Bounds,
    content: TypesetElementContent,
}

impl TypesetElement {
    pub fn new(bounds: Bounds, content: TypesetElementContent) -> Self {
        Self {
            id: ElementId::new(),
            bounds,
            content,
        }
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn content(&self) -> &TypesetElementContent {
        &self.content
    }
}

#[derive(Debug)]
pub enum TypesetElementContent {
    Group(TypesetElementGroup),
    TextSlice(TextSliceContent),
    Image,
}

#[derive(Debug)]
pub struct TypesetElementGroup {
    pub elements: Vec<TypesetElement>,
}

#[derive(Debug)]
pub struct TextSliceContent {
    pub text: String,
}
