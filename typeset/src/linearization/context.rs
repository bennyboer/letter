#[derive(Copy, Clone, Debug)]
pub(crate) struct LinearizationContext {
    section_level: usize,
    heading_counter: usize,
}

impl LinearizationContext {
    pub fn new() -> Self {
        Self {
            section_level: 0,
            heading_counter: 0,
        }
    }

    pub fn increase_section(&self) -> Self {
        let mut copy = self.clone();
        copy.section_level += 1;
        copy.heading_counter = 0;
        copy
    }

    pub fn increase_heading_counter(&self) -> Self {
        let mut copy = self.clone();
        copy.heading_counter += 1;
        copy
    }
}
