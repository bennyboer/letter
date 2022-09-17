use unit::{Distance, DistanceUnit};

use crate::element::{Page, PageConstraints, Position, Size, TypesetElement};

pub(crate) struct TypesettingContext {
    pages: Vec<Page>,
    offset: Position,
    page_constraints: PageConstraints,
}

impl TypesettingContext {
    pub fn new() -> Self {
        let page_size = Size::new(
            Distance::new(210.0, DistanceUnit::Millimeter),
            Distance::new(297.0, DistanceUnit::Millimeter),
        ); // TODO Make page size configurable

        Self {
            pages: Vec::new(),
            offset: Position::zero(),
            page_constraints: PageConstraints {
                size: page_size,
                top: Distance::new(20.0, DistanceUnit::Millimeter), // TODO Make page margin configurable
                bottom: Distance::new(20.0, DistanceUnit::Millimeter),
                left: Distance::new(20.0, DistanceUnit::Millimeter),
                right: Distance::new(20.0, DistanceUnit::Millimeter),
            },
        }
    }

    pub fn new_page(&mut self) {
        let next_page_number = self.last_page_number() + 1;
        self.pages.push(Page::new(next_page_number));
        self.offset = Position::zero();
    }

    pub fn add_element_to_page(&mut self, element: TypesetElement) -> ElementAddResult {
        let position = element.bounds().position();
        let size = element.bounds().size();

        let available_height = self.available_height_on_current_page();
        if available_height < size.height {
            return ElementAddResult::NotEnoughSpaceAvailableOnPage {
                element,
                available_height,
            };
        }

        self.offset = Position::absolute(position.x() + size.width, position.y() + size.height);

        // TODO Not sure if we should block the whole height of the page here or take the actual width of the element to add into account?
        self.page_constraints.top += size.height;

        let page = self.last_page();
        page.add_element(element);

        ElementAddResult::Success
    }

    pub fn last_page(&mut self) -> &mut Page {
        let has_no_page = self.pages.is_empty();
        if has_no_page {
            self.new_page();
        }

        self.pages.last_mut().unwrap()
    }

    pub fn last_page_number(&self) -> usize {
        self.pages.last().map_or(0, |p| p.number())
    }

    pub fn current_offset(&self) -> Position {
        self.offset
    }

    pub fn pages(self) -> Vec<Page> {
        self.pages
    }

    fn available_height_on_current_page(&self) -> Distance {
        let total_height = self.page_constraints.size.height;
        total_height - self.page_constraints.top - self.page_constraints.bottom
    }
}

pub(crate) enum ElementAddResult {
    Success,
    NotEnoughSpaceAvailableOnPage {
        element: TypesetElement,
        available_height: Distance,
    },
}
