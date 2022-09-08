pub use page::Page;
pub use position::Position;
pub use size::Size;

mod page;
mod position;
mod size;

#[derive(Debug)]
pub struct Bounds {
    pub position: Position,
    pub size: Size,
}

#[derive(Debug)]
pub struct TypesetElement {
    pub bounds: Bounds,
    pub content: TypesetElementContent,
}

#[derive(Debug)]
pub enum TypesetElementContent {
    TextSlice(TextSliceContent),
    Image,
}

#[derive(Debug)]
pub struct TextSliceContent {
    pub text: String,
}
