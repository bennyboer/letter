use std::error::Error;

pub type TypesetResult<T> = Result<T, Box<dyn Error>>;
