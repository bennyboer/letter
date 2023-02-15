pub use text_slice::TextSliceContent;

mod text_slice;

#[derive(Debug)]
pub enum LayoutElementContent {
    Page,
    TextSlice(TextSliceContent),
    Image,
}
