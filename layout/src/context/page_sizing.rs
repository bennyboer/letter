use crate::element::LayoutConstraints;

pub(crate) trait PageSizing {
    fn get_page_constraints(&self, page_number: usize) -> LayoutConstraints;
}

pub(crate) struct OneSizeFitsAllPageSizing {
    constraints: LayoutConstraints,
}

impl OneSizeFitsAllPageSizing {
    pub(crate) fn new(constraints: LayoutConstraints) -> Self {
        Self { constraints }
    }
}

impl PageSizing for OneSizeFitsAllPageSizing {
    fn get_page_constraints(&self, _page_number: usize) -> LayoutConstraints {
        self.constraints
    }
}
