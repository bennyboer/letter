use document::structure::{
    DocumentNode, DocumentNodeValue, DocumentStructure, NodeId, SourcePosition, SourceSpan,
};
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
    parent_node_id: NodeId,
    document_structure: &mut DocumentStructure,
) {
    let mut path = document_structure.get_path(parent_node_id);
    path.reverse();
    let has_paragraph_ancestor = path
        .iter()
        .find(|node| match node.value {
            DocumentNodeValue::Paragraph => true,
            _ => false,
        })
        .is_some();

    let create_paragraph_parent = !has_paragraph_ancestor;
    let parent_node_id = if create_paragraph_parent {
        let document_node_id = document_structure.unused_node_id();
        let value = DocumentNodeValue::Paragraph;
        let source_span = document_structure
            .get_node(parent_node_id)
            .expect("A node at this point must have a parent")
            .source_span;
        let document_node = DocumentNode::new(document_node_id, value, source_span);

        document_structure.insert(parent_node_id, document_node);

        document_node_id
    } else {
        parent_node_id
    };

    let document_node_id = document_structure.unused_node_id();
    let value = DocumentNodeValue::Text(text.to_string());
    let document_node = DocumentNode::new(document_node_id, value, None);

    document_structure.insert(parent_node_id, document_node);
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
        let source_span = SourceSpan::new(
            SourcePosition::new(
                element.source_span.start_line,
                element.source_span.start_column,
            ),
            SourcePosition::new(element.source_span.end_line, element.source_span.end_column),
        );
        let document_node = DocumentNode::new(document_node_id, value, Some(source_span));
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

            // TODO: Include child nodes from given src when src is given

            DocumentNodeValue::Section
        }
        "heading" | "h" => DocumentNodeValue::Heading,
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
    [Text(\"\\n    Hello\\n\")]
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
    fn should_parse_paragraph_with_some_formatting() {
        // given: an input letter script format with a paragraph and some formatting
        let src = r#"
<p>
    This is a simple paragraph with some <b>formatting</b>.
    Additionally it <i>con</i>tains wei<i>rd</i> formatting choices!
</p>
"#;

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the resulting structure should look like the following
        assert_eq!(
            structure.fmt_pretty(),
            r#"[DocumentRoot]
  [Paragraph]
    [Text("\n    This is a simple paragraph with some ")]
    [Custom("b")]
      [Text("formatting")]
    [Text(".\n    Additionally it ")]
    [Custom("i")]
      [Text("con")]
    [Text("tains wei")]
    [Custom("i")]
      [Text("rd")]
    [Text(" formatting choices!\n")]
"#
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
        );
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
        );
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
    [Text(\"\\n    This is some text!\\n\")]
  [Paragraph]
    [Text(\"\\n    This is another paragraph.\\n\")]
"
        );
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
    [Text(\"Hello World!\\n\\n\")]
  [Paragraph]
    [Text(\"\\n    This is another example\\n    with some \")]
    [Custom(\"b\")]
      [Text(\"bold\")]
    [Text(\" text\\n    to verify that bold will not get a paragraph\\n    parent added.\\n\")]
"
        );
    }

    #[test]
    fn should_parse_sections_and_headings() {
        // given: an input letter script format with some sections and headings
        let src = "\
<heading> My document title </heading>

<section>

    <h> A first-level heading </h>

    <p>
        Some paragraphs text.
    </p>

    <section>
    
        <heading> A second-level heading </heading>

    </section>

</section>";

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the sections and headings should be parsed as follows
        assert_eq!(
            structure.fmt_pretty(),
            "\
[DocumentRoot]
  [Heading]
    [Paragraph]
      [Text(\" My document title \")]
  [Section]
    [Heading]
      [Paragraph]
        [Text(\" A first-level heading \")]
    [Paragraph]
      [Text(\"\\n        Some paragraphs text.\\n    \")]
    [Section]
      [Heading]
        [Paragraph]
          [Text(\" A second-level heading \")]
"
        );
    }

    #[test]
    fn should_parse_an_image() {
        // given: an input letter script format with an image element
        let src = r#"<img src="my-image.png" />

<img src="another.png" width="2cm" height="3cm" />

<img src="only-height.png" height="10cm" />

<img src="only-width.png" width="6cm" />"#;

        // when: the document structure is parsed
        let structure = parse_document_structure(src).unwrap();

        // then: the sections and headings should be parsed as follows
        assert_eq!(
            structure.fmt_pretty(),
            r#"[DocumentRoot]
  [Image { source: "my-image.png", width: None, height: None }]
  [Image { source: "another.png", width: Some("2cm"), height: Some("3cm") }]
  [Image { source: "only-height.png", width: None, height: Some("10cm") }]
  [Image { source: "only-width.png", width: Some("6cm"), height: None }]
"#
        );
    }
}
