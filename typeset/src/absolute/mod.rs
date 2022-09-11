mod context;

use log::info;

use crate::{element::Page, linearization::Block, relative, result::TypesetResult};

use context::TypesettingContext;

use self::context::ElementAddResult;

pub(crate) fn typeset_absolutely(blocks: &[Block]) -> TypesetResult<Vec<Page>> {
    let mut iteration = 0;
    loop {
        info!(
            "[Typesetting] Absolute typesetting iteration {}",
            iteration + 1
        );

        let pages = layout_blocks_to_pages(blocks)?;

        let references_stable = check_references_stable();
        if references_stable {
            return Ok(pages);
        }

        iteration += 1;
    }
}

fn layout_blocks_to_pages(blocks: &[Block]) -> TypesetResult<Vec<Page>> {
    let mut ctx = TypesettingContext::new();

    for block in blocks {
        let element = relative::typeset_relatively(block)?;

        if let ElementAddResult::NotEnoughSpaceAvailableOnPage {
            element,
            available_height: _,
        } = ctx.add_element_to_page(element)
        {
            // TODO Should retry relative typesetting with constraints (if the blocks typesetter supports that), otherwise just break the page
            ctx.new_page();
            ctx.add_element_to_page(element);
        }
    }

    Ok(ctx.pages())
}

// TODO Do once references are implemented
fn check_references_stable() -> bool {
    true
}
