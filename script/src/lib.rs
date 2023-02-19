use std::error::Error;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;

use document::structure::{
    DocumentNode, DocumentNodeValue, DocumentStructure, NodeId, SourcePosition,
};
use source_position_resolver::SourcePositionResolver;

mod source_position_resolver;

pub type ParseResult<T> = Result<T, Box<dyn Error>>;

struct ParseContext {
    document_structure: DocumentStructure,
    node_stack: Vec<NodeId>,
    source_position_resolver: SourcePositionResolver,
}

/// Parsing a document structure from the letter script format.
/// This is done without a custom written parser since we only
/// expect something like HTML/XML.
pub fn parse_document_structure(src: &str) -> ParseResult<DocumentStructure> {
    let source_position_resolver = SourcePositionResolver::from_str(src);
    let mut context = ParseContext {
        document_structure: DocumentStructure::new(),
        node_stack: Vec::new(),
        source_position_resolver,
    };
    context
        .node_stack
        .push(context.document_structure.root().id);

    let mut reader = Reader::from_str(src); // TODO Read from file instead for better performance?
    reader.trim_text(true);

    let mut buffer = Vec::new();
    loop {
        let event = reader.read_event_into(&mut buffer);
        match event {
            Err(e) => Err(format!(
                "Parse error at position {}: {:?}",
                reader.buffer_position(),
                e
            )
            .to_owned())?,
            Ok(event) => match event {
                Event::Eof => break,
                _ => handle_event(event, reader.buffer_position(), &reader, &mut context)?,
            },
        };

        buffer.clear();
    }

    if context.node_stack.len() != 1 {
        return Err("Unclosed tags found".into());
    }

    Ok(context.document_structure)
}

fn handle_event(
    event: Event,
    offset: usize,
    reader: &Reader<&[u8]>,
    context: &mut ParseContext,
) -> ParseResult<()> {
    match event {
        Event::Start(e) => enter_child_node(
            reader.decoder().decode(e.local_name().as_ref())?.as_ref(),
            e.attributes(),
            offset,
            context,
        ),
        Event::End(e) => leave_child_node(
            reader.decoder().decode(e.local_name().as_ref())?.as_ref(),
            context,
        ),
        Event::Empty(e) => push_child_node(
            reader.decoder().decode(e.local_name().as_ref())?.as_ref(),
            e.attributes(),
            offset,
            context,
        ),
        Event::Text(e) => push_text_node(e.unescape()?.into_owned(), offset, context),
        _ => Ok(()),
    }
}

fn push_text_node(text: String, offset: usize, context: &mut ParseContext) -> ParseResult<()> {
    let node = to_text_node(text, offset, context);
    insert_node(node, context);

    Ok(())
}

fn push_child_node(
    name: &str,
    attributes: Attributes,
    offset: usize,
    context: &mut ParseContext,
) -> ParseResult<()> {
    let node = to_node(name, offset, attributes, context)?;
    insert_node(node, context);

    Ok(())
}

fn leave_child_node(_name: &str, context: &mut ParseContext) -> ParseResult<()> {
    context
        .node_stack
        .pop()
        .expect("No node found on node stack");

    Ok(())
}

fn enter_child_node(
    name: &str,
    attributes: Attributes,
    offset: usize,
    context: &mut ParseContext,
) -> ParseResult<()> {
    let node = to_node(name, offset, attributes, context)?;
    let node_id = node.id;
    insert_node(node, context);

    context.node_stack.push(node_id);

    Ok(())
}

fn find_source_position(offset: usize, context: &mut ParseContext) -> SourcePosition {
    context
        .source_position_resolver
        .lookup(offset)
        .expect("Source position of node not found")
}

fn insert_node(node: DocumentNode, context: &mut ParseContext) {
    let parent_id = context
        .node_stack
        .last()
        .expect("No parent node found for text node");
    context.document_structure.insert(*parent_id, node);
}

fn to_node(
    name: &str,
    offset: usize,
    attributes: Attributes,
    context: &mut ParseContext,
) -> ParseResult<DocumentNode> {
    let source_position = find_source_position(offset, context);

    let node_id = context.document_structure.unused_node_id();
    let node_value = to_node_value(name, attributes, source_position)?;

    Ok(DocumentNode::new(
        node_id,
        Some(name.to_owned()),
        node_value,
        Some(source_position),
    ))
}

fn to_text_node(text: String, offset: usize, context: &mut ParseContext) -> DocumentNode {
    let source_position = find_source_position(offset, context);

    let node_id = context.document_structure.unused_node_id();

    DocumentNode::new(
        node_id,
        None,
        DocumentNodeValue::Text(text),
        Some(source_position),
    )
}

fn to_node_value(
    name: &str,
    _attributes: Attributes,
    source_position: SourcePosition,
) -> ParseResult<DocumentNodeValue> {
    // TODO Parse attributes

    Ok(match name {
        "section" | "s" => DocumentNodeValue::Section,
        "paragraph" | "p" => DocumentNodeValue::Paragraph,
        "heading" | "h" => DocumentNodeValue::Heading,
        "list" | "l" => DocumentNodeValue::List,
        "list-item" | "li" => DocumentNodeValue::ListItem,
        "table" | "t" => DocumentNodeValue::Table,
        "break" | "br" => DocumentNodeValue::Break,
        "image" | "img" => DocumentNodeValue::Image {
            source: "".to_string(), // TODO Parse from attributes
            width: None,
            height: None,
        },
        _ => Err(format!(
            "Node with name '{}' at '{}:{}' is currently not supported.",
            name, source_position.line, source_position.column
        )
        .to_owned())?,
    })
}
