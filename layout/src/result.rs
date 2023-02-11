use std::error::Error;

pub type LayoutResult<T> = Result<T, Box<dyn Error>>;
