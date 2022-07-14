use comrak::{markdown_to_html, ComrakOptions, ComrakParseOptions, ComrakRenderOptions};
use document::structure::{DocumentNode, DocumentNodeValue, DocumentStructure, NodeId};
use html_parser::{Dom, Element, Node};
use std::error::Error;

type ParseResult<T> = Result<T, Box<dyn Error>>;

/// Parsing a document structure from the letter script format.
/// This is done without a custom written parser since we only expect something
/// like HTML/XML and/or Markdown.
/// Thus we use a HTML parser to initially prepare the document tree followed by
/// a Markdown parser that will parse every text node that the HTML parser has found.
/// Afterwards the results of the Markdown parser need to be post-processed by the HTML
/// parser again since it outputs HTML.
pub fn parse_document_structure(src: &str) -> ParseResult<DocumentStructure> {
    let dom = Dom::parse(src)?;

    // println!(
    //     "[DOM] Parsing DOM succeeded -> {}",
    //     dom.to_json_pretty().unwrap()
    // );

    let mut document_structure = DocumentStructure::new();
    fill_dom_in_document_structure(&dom, &mut document_structure)?;

    // TODO Tree shaking that removes unnecessary elements (nested <p> elements (concatenate classes))

    Ok(document_structure)
}

fn fill_dom_in_document_structure(
    dom: &Dom,
    document_structure: &mut DocumentStructure,
) -> ParseResult<()> {
    for node in &dom.children {
        fill_node_in_document_structure(
            node,
            document_structure.root().id,
            document_structure,
            false,
            false,
        )?;
    }

    Ok(())
}

fn fill_node_in_document_structure(
    node: &Node,
    parent_node: NodeId,
    document_structure: &mut DocumentStructure,
    markdown_already_processed: bool,
    in_paragraph: bool,
) -> ParseResult<()> {
    match node {
        Node::Text(text) => {
            fill_text_node_to_document_structure(
                text,
                parent_node,
                document_structure,
                markdown_already_processed,
                in_paragraph,
            )?;
        }
        Node::Element(element) => fill_element_to_document_structure(
            element,
            parent_node,
            document_structure,
            markdown_already_processed,
            in_paragraph,
        )?,
        _ => {}
    }

    Ok(())
}

fn fill_text_node_to_document_structure(
    text: &String,
    parent_node: NodeId,
    document_structure: &mut DocumentStructure,
    markdown_already_processed: bool, // TODO Extract document structure and in_paragraph as flags, parent_node and document_structure to context struct!
    in_paragraph: bool,
) -> ParseResult<()> {
    if markdown_already_processed {
        let document_node_id = document_structure.unused_node_id();
        let value = DocumentNodeValue::Text(text.to_string());
        let document_node = DocumentNode::new(document_node_id, value);

        document_structure.insert(parent_node, document_node);
    } else {
        let paragraphs = split_text_to_paragraphs(text);

        for paragraph in paragraphs {
            let html = markdown_to_html(&paragraph, &ComrakOptions::default());
            let dom = Dom::parse(&html)?;

            for child_node in dom.children {
                fill_node_in_document_structure(
                    &child_node,
                    parent_node,
                    document_structure,
                    true,
                    in_paragraph,
                )?;
            }
        }
    }

    // TODO Remove <p> under <heading>, ...
    // TODO Copy element to document structure

    Ok(())
}

fn fill_element_to_document_structure(
    element: &Element,
    parent_node: NodeId,
    document_structure: &mut DocumentStructure,
    markdown_already_processed: bool,
    in_paragraph: bool,
) -> ParseResult<()> {
    let value = map_element_to_document_node_value(element);
    let is_paragraph_node = match value {
        DocumentNodeValue::Paragraph | DocumentNodeValue::Header => true,
        _ => false,
    };
    let skip_inserting_document_node = is_paragraph_node && in_paragraph;
    let in_paragraph = if is_paragraph_node {
        true
    } else {
        in_paragraph
    };

    let document_node_id = if skip_inserting_document_node {
        parent_node
    } else {
        let document_node_id = document_structure.unused_node_id();
        let document_node = DocumentNode::new(document_node_id, value);
        document_structure.insert(parent_node, document_node);

        document_node_id
    };

    for child_node in &element.children {
        fill_node_in_document_structure(
            child_node,
            document_node_id,
            document_structure,
            markdown_already_processed,
            in_paragraph,
        )?;
    }

    Ok(())
}

fn map_element_to_document_node_value(element: &Element) -> DocumentNodeValue {
    match element.name.as_str() {
        "section" => DocumentNodeValue::Section,
        "header" | "h1" | "h2" | "h3" | "h4" | "h5" | "h62" => DocumentNodeValue::Header,
        "p" | "paragraph" => DocumentNodeValue::Paragraph,
        "img" | "image" => DocumentNodeValue::Image {
            source: element.attributes["src"]
                .clone()
                .expect("An image must specify a 'src' attribute"),
            width: element.attributes["width"].clone(),
            height: element.attributes["height"].clone(),
        },
        _ => DocumentNodeValue::Custom(element.name.to_string()),
    }
}

fn split_text_to_paragraphs(text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut buffer = String::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            paragraphs.push(buffer.to_string());
            buffer.clear();
        } else {
            let add_new_line = !buffer.is_empty();
            if add_new_line {
                buffer += "\n";
            }
            buffer += line.trim();
        }
    }
    if !buffer.is_empty() {
        paragraphs.push(buffer.to_string());
    }

    paragraphs
}
