use std::error::Error;

pub type StyleParseResult<T> = Result<T, Box<dyn Error>>;
