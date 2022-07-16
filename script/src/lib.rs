use document::structure::{DocumentNode, DocumentNodeValue, DocumentStructure, NodeId};
use html_parser::{Dom, Element, Node};
use log::warn;
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
    let mut path = document_structure.get_path(parent_node);
    path.reverse();
    let has_paragraph_ancestor = path
        .iter()
        .find(|node| match node.value {
            DocumentNodeValue::Paragraph => true,
            _ => false,
        })
        .is_some();

    let create_paragraph_parent = !has_paragraph_ancestor;
    let parent_node = if create_paragraph_parent {
        let document_node_id = document_structure.unused_node_id();
        let value = DocumentNodeValue::Paragraph;
        let document_node = DocumentNode::new(document_node_id, value);

        document_structure.insert(parent_node, document_node);

        document_node_id
    } else {
        parent_node
    };

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
        let value = map_element_to_document_node_value(element, parent_node, document_structure);
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

fn map_element_to_document_node_value(
    element: &Element,
    parent_node: NodeId,
    document_structure: &DocumentStructure,
) -> DocumentNodeValue {
    match element.name.as_str() {
        "section" | "s" => {
            let source = element.attributes.get("src").and_then(|o| o.clone());
            if source.is_some() && !element.children.is_empty() {
                warn!(
                    "Found <section> element at lines {}-{} with 'src' attribute that still has content \
                that will not find its way into the output. You may consider either removing \
                the content of the <section> element or removing the 'src' attribute to get \
                rid of this warning.",
                    element.source_span.start_line,
                    element.source_span.end_line
                )
            }
            DocumentNodeValue::Section { source }
        }
        "header" | "h" => DocumentNodeValue::Header,
        "paragraph" | "p" => DocumentNodeValue::Paragraph,
        "image" | "img" => DocumentNodeValue::Image {
            source: element
                .attributes
                .get("src")
                .expect("Could not find 'src' attribute for image element")
                .clone()
                .expect("Value missing for 'src' attribute of image element"),
            width: element.attributes.get("width").and_then(|o| o.clone()),
            height: element.attributes.get("height").and_then(|o| o.clone()),
        },
        "list" => DocumentNodeValue::List,
        "item" => {
            let is_parent_of_type_list =
                document_structure
                    .get_node(parent_node)
                    .map_or(false, |node| match node.value {
                        DocumentNodeValue::List => true,
                        _ => false,
                    });
            if is_parent_of_type_list {
                DocumentNodeValue::ListItem
            } else {
                DocumentNodeValue::Custom(element.name.to_string())
            }
        }
        _ => DocumentNodeValue::Custom(element.name.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_ignore_document_element() {
        // given: an input letter script format with a root document element and some text
        let src = "\
<document>
    Hello
</document>";

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the structure should contain the document root node
        assert_eq!(
            structure.fmt_pretty(),
            "\
[DocumentRoot]
  [Paragraph]
    [Text(\"Hello\")]
"
        );
    }

    #[test]
    fn should_not_require_document_element() {
        // given: an input letter script format with no root document element and only some text
        let src = "Hello";

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the resulting structure should include a document root node
        assert_eq!(
            structure.fmt_pretty(),
            "\
[DocumentRoot]
  [Paragraph]
    [Text(\"Hello\")]
"
        );
    }

    #[test]
    fn should_parse_flat_list() {
        // given: an input letter script format with a flat list element
        let src = "\
<list>
    <item>First item</item>
    <item>Second item</item>
    <item>Third item</item>            
</list>";

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the resulting structure should include the list element and its children items
        assert_eq!(
            structure.fmt_pretty(),
            "\
[DocumentRoot]
  [List]
    [ListItem]
      [Paragraph]
        [Text(\"First item\")]
    [ListItem]
      [Paragraph]
        [Text(\"Second item\")]
    [ListItem]
      [Paragraph]
        [Text(\"Third item\")]
"
        )
    }

    #[test]
    fn should_parse_nested_list() {
        // given: an input letter script format with a nested list element
        let src = "\
<list>
    <item>First item</item>
    <item>Second item</item>
    <list>
        <item>A nested list item</item>
        <item>Another one!</item>
    </list>
    <item>Third item</item>            
</list>";

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the resulting structure should include the list element and its children items
        assert_eq!(
            structure.fmt_pretty(),
            "\
[DocumentRoot]
  [List]
    [ListItem]
      [Paragraph]
        [Text(\"First item\")]
    [ListItem]
      [Paragraph]
        [Text(\"Second item\")]
    [List]
      [ListItem]
        [Paragraph]
          [Text(\"A nested list item\")]
      [ListItem]
        [Paragraph]
          [Text(\"Another one!\")]
    [ListItem]
      [Paragraph]
        [Text(\"Third item\")]
"
        )
    }

    #[test]
    fn should_parse_text_in_paragraphs() {
        // given: an input letter script format with some text
        let src = "\
<paragraph>
    This is some text!
</paragraph>

<p>
    This is another paragraph.
</p>";

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the resulting structure should include the text in the given paragraphs.
        assert_eq!(
            structure.fmt_pretty(),
            "\
[DocumentRoot]
  [Paragraph]
    [Text(\"This is some text!\")]
  [Paragraph]
    [Text(\"This is another paragraph.\")]
"
        )
    }

    #[test]
    fn should_include_paragraph_when_text_is_encountered_without_paragraph_parent() {
        // given: an input letter script format with some text without paragraph
        let src = "\
Hello World!

<p>
    This is another example
    with some <b>bold</b> text
    to verify that bold will not get a paragraph
    parent added.
</p>";

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the text "Hello World!" should have a parent paragraph element
        assert_eq!(
            structure.fmt_pretty(),
            "\
[DocumentRoot]
  [Paragraph]
    [Text(\"Hello World!\")]
  [Paragraph]
    [Text(\"This is another example\\n    with some\")]
    [Custom(\"b\")]
      [Text(\"bold\")]
    [Text(\"text\\n    to verify that bold will not get a paragraph\\n    parent added.\")]
"
        )
    }
}
