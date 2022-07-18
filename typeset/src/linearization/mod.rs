/// Linearization of a document structure is the process
/// of turning a document tree into a flat list of "blocks".
/// Blocks in turn are an abstract representation of a group
/// of elements that belong together in a document.
/// An example for a block would be a paragraph that contains of words/sentences/glyphs
/// or an image together with a caption, etc.
mod block;
mod context;

use crate::linearization::block::image::ImageBlock;
use crate::linearization::block::list::ListBlock;
use crate::linearization::block::text::{
    TextBlock, TextBlockPart, TextBlockPartTextValue, TextBlockPartValue,
};
use crate::linearization::block::BlockValue;
use crate::TypesetResult;
pub(crate) use block::Block;
use context::LinearizationContext;
use document::structure::{DocumentNode, DocumentNodeValue, DocumentStructure, NodeId};
use log::error;
use std::process::exit;

pub(crate) fn linearize(document_structure: &DocumentStructure) -> TypesetResult<Vec<Block>> {
    let mut result = Vec::new();
    let ctx = LinearizationContext::new();

    linearize_children(
        document_structure.root(),
        document_structure,
        &mut result,
        ctx,
    )?;

    Ok(result)
}

fn linearize_children(
    node: &DocumentNode,
    document_structure: &DocumentStructure,
    blocks: &mut Vec<Block>,
    ctx: LinearizationContext,
) -> TypesetResult<()> {
    for child_node in &node.children {
        linearize_node(*child_node, document_structure, blocks, ctx)?;
    }

    Ok(())
}

fn linearize_node(
    node_id: NodeId,
    document_structure: &DocumentStructure,
    blocks: &mut Vec<Block>,
    ctx: LinearizationContext,
) -> TypesetResult<()> {
    if let Some(node) = document_structure.get_node(node_id) {
        match &node.value {
            DocumentNodeValue::Text(..) => panic!("Something is wrong here. A text node mustn't be outside of a paragraph node. Please file a bug report."),
            DocumentNodeValue::Section => {
                let ctx = ctx.increase_section();
                linearize_children(node, document_structure, blocks, ctx)?
            }
            DocumentNodeValue::Heading => {
                let ctx = ctx.increase_heading_counter();
                linearize_children(node, document_structure, blocks, ctx)?
            }
            DocumentNodeValue::Paragraph => {
                let block_value = linearize_paragraph(node, document_structure)?;
                blocks.push(Block::new(node_id, block_value));
            }
            DocumentNodeValue::Image { .. } => {
                let block_value = linearize_image(node, document_structure)?;
                blocks.push(Block::new(node_id, block_value));
            }
            DocumentNodeValue::List => {
                let block_value = linearize_list(node, document_structure)?;
                blocks.push(Block::new(node_id, block_value));
            }
            DocumentNodeValue::ListItem => panic!("Something is wrong here. A list imte node mustn't be outside of a list node. Please file a bug report."),
            _ => linearize_children(node, document_structure, blocks, ctx)?
        }
    }

    Ok(())
}

fn linearize_paragraph(
    node: &DocumentNode,
    document_structure: &DocumentStructure,
) -> TypesetResult<BlockValue> {
    let mut parts = Vec::new();

    for child_node in &node.children {
        linearize_paragraph_node(*child_node, document_structure, &mut parts)?;
    }

    Ok(BlockValue::Text(TextBlock { parts }))
}

fn linearize_paragraph_node(
    node_id: NodeId,
    document_structure: &DocumentStructure,
    parts: &mut Vec<TextBlockPart>,
) -> TypesetResult<()> {
    if let Some(node) = document_structure.get_node(node_id) {
        match &node.value {
            DocumentNodeValue::Text(text) => {
                let part_value = TextBlockPartValue::Text(TextBlockPartTextValue {
                    value: text.to_string(),
                });
                let part = TextBlockPart {
                    value: part_value,
                    document_structure_node: node_id,
                };
                parts.push(part);
            }
            DocumentNodeValue::Image { .. } => panic!("Inline images are not yet supported"), // TODO
            DocumentNodeValue::Custom(_) => {
                for child_node in &node.children {
                    linearize_paragraph_node(*child_node, document_structure, parts)?;
                }
            }
            _ => match node.source_span {
                None => panic!("Encountered unsupported element in paragraph"),
                Some(source_span) => {
                    error!(
                        "Encountered unsupported element in paragraph at lines {}-{}",
                        source_span.start.line, source_span.end.line
                    );
                    exit(1);
                }
            },
        }
    }

    Ok(())
}

fn linearize_image(
    _node: &DocumentNode,
    _document_structure: &DocumentStructure,
) -> TypesetResult<BlockValue> {
    // TODO Read properties from nodes value and load image to heap

    Ok(BlockValue::Image(ImageBlock {}))
}

fn linearize_list(
    _node: &DocumentNode,
    _document_structure: &DocumentStructure,
) -> TypesetResult<BlockValue> {
    // TODO Read child list item nodes and linearize the content per item (could be one or multiple paragraphs, images, ...)

    Ok(BlockValue::List(ListBlock {}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linearization::block::text::TextBlockPartValue;
    use crate::linearization::block::BlockValue;
    use script::parse_document_structure;

    #[test]
    fn should_linearize_simple_paragraph() {
        // given: a document structure that contains a simple text paragraph without formatting
        let script = r#"
<p>
    Hi there, this is a simple paragraph!
    It contains two sentences and no formatting.
</p>
"#;
        let structure = parse_document_structure(script).unwrap();

        // when: the document structure is linearized
        let blocks = linearize(&structure).unwrap();

        // then: the result contains a text block
        assert_eq!(
            blocks.len(),
            1,
            "Expected to have only one block, instead found multiple"
        );
        let block = blocks.first().unwrap();
        let text_block = if let BlockValue::Text(text_block) = &block.value {
            text_block
        } else {
            panic!("Expected the block to be a text block");
        };

        // and: the text block contains only one part that is the paragraphs content
        assert_eq!(
            text_block.parts.len(),
            1,
            "Expected to have only one part in the text block"
        );
        let text_value = if let TextBlockPartValue::Text(text_value) =
            &text_block.parts.first().unwrap().value
        {
            text_value
        } else {
            panic!("Expected the text blocks only part to be a simple text value");
        };
        assert_eq!(
            text_value.value,
            "
    Hi there, this is a simple paragraph!
    It contains two sentences and no formatting.
"
        );
    }

    #[test]
    fn should_linearize_paragraph_with_formatting() {
        // given: a document structure that contains a text paragraph with formatting
        let script = r#"
<p>
    Hi there, this is a <b>formatted</b> paragraph!
    It contains two <strong>sen</strong>tences and <i>some</i> formatting.
</p>
"#;
        let structure = parse_document_structure(script).unwrap();

        // when: the document structure is linearized
        let blocks = linearize(&structure).unwrap();

        // then: the result contains a text block
        let block = blocks.first().unwrap();
        let text_block = if let BlockValue::Text(text_block) = &block.value {
            text_block
        } else {
            panic!("Expected the block to be a text block");
        };

        // and: the text block contains 7 text parts
        assert_eq!(
            text_block.parts.len(),
            7,
            "Expected to have 7 parts in the text block"
        );
        let parts_strings: Vec<String> = text_block
            .parts
            .iter()
            .map(|p| match &p.value {
                TextBlockPartValue::Text(value) => String::from(&value.value),
                _ => "".to_string(),
            })
            .collect();
        assert_eq!(
            parts_strings,
            vec![
                "\n    Hi there, this is a ",
                "formatted",
                " paragraph!\n    It contains two ",
                "sen",
                "tences and ",
                "some",
                " formatting.\n"
            ]
        );
    }
}
