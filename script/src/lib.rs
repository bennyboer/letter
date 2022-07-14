use document::structure::{DocumentNode, DocumentNodeValue, DocumentStructure, NodeId};
use html_parser::{Dom, Element, Node};
use std::error::Error;

pub type ParseResult<T> = Result<T, Box<dyn Error>>;

/// Parsing a document structure from the letter script format.
/// This is done without a custom written parser since we only
/// expect something like HTML/XML.
pub fn parse_document_structure(src: &str) -> ParseResult<DocumentStructure> {
    let dom = Dom::parse(src)?;

    let mut document_structure = DocumentStructure::new();
    fill_dom_in_document_structure(&dom, &mut document_structure);

    Ok(document_structure)
}

fn fill_dom_in_document_structure(dom: &Dom, document_structure: &mut DocumentStructure) {
    for node in &dom.children {
        fill_node_in_document_structure(node, document_structure.root().id, document_structure);
    }
}

fn fill_node_in_document_structure(
    node: &Node,
    parent_node: NodeId,
    document_structure: &mut DocumentStructure,
) {
    match node {
        Node::Text(text) => {
            fill_text_node_to_document_structure(text, parent_node, document_structure);
        }
        Node::Element(element) => {
            fill_element_to_document_structure(element, parent_node, document_structure);
        }
        _ => {}
    }
}

fn fill_text_node_to_document_structure(
    text: &String,
    parent_node: NodeId,
    document_structure: &mut DocumentStructure,
) {
    let document_node_id = document_structure.unused_node_id();
    let value = DocumentNodeValue::Text(text.to_string());
    let document_node = DocumentNode::new(document_node_id, value);

    document_structure.insert(parent_node, document_node);
}

fn fill_element_to_document_structure(
    element: &Element,
    parent_node: NodeId,
    document_structure: &mut DocumentStructure,
) {
    let skip_element = is_element_to_skip(element);
    let next_parent_node_id = if skip_element {
        parent_node
    } else {
        let document_node_id = document_structure.unused_node_id();
        let value = map_element_to_document_node_value(element);
        let document_node = DocumentNode::new(document_node_id, value);
        document_structure.insert(parent_node, document_node);
        document_node_id
    };

    for child_node in &element.children {
        fill_node_in_document_structure(child_node, next_parent_node_id, document_structure);
    }
}

fn is_element_to_skip(element: &Element) -> bool {
    let name = element.name.as_str();
    match name {
        "document" => true,
        _ => false,
    }
}

fn map_element_to_document_node_value(element: &Element) -> DocumentNodeValue {
    match element.name.as_str() {
        "section" => DocumentNodeValue::Section,
        "header" => DocumentNodeValue::Header,
        "p" | "paragraph" => DocumentNodeValue::Paragraph,
        "img" | "image" => DocumentNodeValue::Image {
            source: element
                .attributes
                .get("src")
                .expect("Could not find 'src' attribute for image element")
                .clone()
                .expect("Value missing for 'src' attribute of image element"),
            width: element.attributes.get("width").and_then(|o| o.clone()),
            height: element.attributes.get("height").and_then(|o| o.clone()),
        },
        _ => DocumentNodeValue::Custom(element.name.to_string()),
    }
}
