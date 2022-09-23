use unit::Distance;

use crate::{
    context::{ElementAddResult, TypesettingContext},
    element::{DocumentLayout, Position},
    linearization::Block,
    relative,
    result::TypesetResult,
};

pub(crate) fn typeset_absolutely(blocks: &[Block]) -> TypesetResult<DocumentLayout> {
    let mut iteration = 0;
    loop {
        let pages = layout_blocks_to_pages(blocks)?;

        let layout_stable = check_layout_stable();
        if layout_stable {
            return Ok(pages);
        }

        iteration += 1;
    }
}

fn layout_blocks_to_pages(blocks: &[Block]) -> TypesetResult<DocumentLayout> {
    let mut ctx = TypesettingContext::new();
    ctx.new_page();

    let mut anchor = Position::relative_to(
        ctx.current_page().element(),
        Distance::zero(),
        Distance::zero(),
    );
    for block in blocks {
        let element = relative::typeset_relatively(block, anchor, &mut ctx)?;
        anchor = Position::relative_to(
            element.id(),
            Distance::zero(),
            element.bounds().size().height,
        );

        if let ElementAddResult::NotEnoughSpaceAvailableOnPage {
            mut element,
            available_height: _,
        } = ctx.add_element_to_page(element)
        {
            // TODO Should retry relative typesetting with constraints (if the blocks typesetter supports that), otherwise just break the page
            ctx.new_page();
            let new_page_element_id = ctx.current_page().element();
            element.bounds_mut().set_position(Position::relative_to(
                new_page_element_id,
                Distance::zero(),
                Distance::zero(),
            ));
            ctx.add_element_to_page(element);
            // TODO What if the element does not have enough space on the new page either? -> Force add it to page even if it overflows!
        }
    }

    Ok(ctx.to_layout())
}

fn check_layout_stable() -> bool {
    check_references_stable()
}

// TODO Do once references are implemented
fn check_references_stable() -> bool {
    true
}
