#[derive(Debug)]
pub struct LayoutOptions {
    pub max_passes: usize,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self { max_passes: 100 }
    }
}
