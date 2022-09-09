use std::error::Error;

pub type ExportResult<T> = Result<T, Box<dyn Error>>;
