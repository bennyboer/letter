pub use model::DocumentStyles;

mod color;
mod model;

pub fn parse(_src: &str) -> DocumentStyles {
    DocumentStyles::new()
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {}
}
