use comrak::{markdown_to_html, ComrakOptions, ComrakRenderOptions};
use document::structure::DocumentStructure;
use html_parser::{Dom, Node};

/// Parsing a document structure from the letter script format.
/// This is done without a custom written parser since we only expect something
/// like HTML/XML and/or Markdown.
/// Thus we use a HTML parser to initially prepare the document tree followed by
/// a Markdown parser that will parse every text node that the HTML parser has found.
/// Afterwards the results of the Markdown parser need to be post-processed by the HTML
/// parser again since it outputs HTML.
pub fn parse_document_structure(src: &str) {
    let dom = match Dom::parse(src) {
        Ok(dom) => {
            println!(
                "[DOM] Parsing DOM succeeded -> {}",
                dom.to_json_pretty().unwrap()
            );
            dom
        }
        Err(e) => {
            panic!("Could not parse DOM from HTML: '{:?}'", e);
        }
    };

    println!("#########################");

    let mut document_structure = DocumentStructure::new();

    // Fill each node in the document structure
    for node in &dom.children {
        fill_node_in_document_structure(node, &mut document_structure);
    }
}

fn fill_node_in_document_structure(node: &Node, document_structure: &mut DocumentStructure) {
    // TODO: Fill nodes in document structure (copy tree)
    match node {
        Node::Text(text) => {
            // Split text (may contain empty lines) into paragraphs
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

            for (i, paragraph) in paragraphs.iter().enumerate() {
                println!("{i}: {paragraph}");
                let html = markdown_to_html(
                    paragraph,
                    &ComrakOptions {
                        render: ComrakRenderOptions {
                            unsafe_: true,
                            ..ComrakRenderOptions::default()
                        },
                        ..ComrakOptions::default()
                    },
                );
                println!("[Markdown to HTML] {html}");
            }
            println!("----")

            // TODO Rename <p> to <paragraph>, <strong> to <b>, ...
            // TODO Remove <p> under <heading>, ...
        }
        Node::Element(element) => {
            // TODO: Copy element to document structure

            for child_node in &element.children {
                fill_node_in_document_structure(child_node, document_structure);
            }
        }
        _ => {}
    }
}
