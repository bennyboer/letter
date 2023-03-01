extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use pest::iterators::Pairs;
use pest::Parser;

use document::style::{ClassName, DocumentStyles, FontFamilySource, NodeName, Style, StyleDefinition};
use unit::{Distance, DistanceUnit};

use crate::result::StyleParseResult;

mod color;
mod result;

#[derive(Parser)]
#[grammar = "lst.pest"]
pub struct LetterStyleParser;

pub fn parse(src: &str) -> StyleParseResult<DocumentStyles> {
    let mut styles = DocumentStyles::new();

    let pairs = LetterStyleParser::parse(Rule::Stylesheet, src)?;
    for pair in pairs {
        if let Rule::Stylesheet = pair.as_rule() {
            parse_stylesheet(pair.into_inner(), &mut styles)?;
        }
    }

    Ok(styles)
}

fn parse_stylesheet(pairs: Pairs<Rule>, styles: &mut DocumentStyles) -> StyleParseResult<()> {
    for pair in pairs {
        if let Rule::Block = pair.as_rule() {
            parse_block(pair.into_inner(), styles)?;
        }
    }
    Ok(())
}

fn parse_block(pairs: Pairs<Rule>, styles: &mut DocumentStyles) -> StyleParseResult<()> {
    let mut selector = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::Selector => {
                selector = Some(parse_selector(pair.into_inner())?);
            }
            Rule::UnnamedBlock => {
                if let Some(selector) = &selector {
                    let style_definition = parse_style_definition(pair.into_inner())?;
                    for selectable in &selector.selectables {
                        styles.register_style_definition(
                            &selectable.node_name,
                            selectable.class_name.as_ref(),
                            style_definition.clone(),
                        );
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn parse_style_definition(pairs: Pairs<Rule>) -> StyleParseResult<StyleDefinition> {
    let mut result = Vec::new();

    for pair in pairs {
        if let Rule::KeyValuePair = pair.as_rule() {
            let styles_for_key_value_pair = parse_styles_from_key_value_pair(pair.into_inner())?;
            result.extend(styles_for_key_value_pair);
        }
    }

    Ok(StyleDefinition { styles: result })
}

fn parse_styles_from_key_value_pair(pairs: Pairs<Rule>) -> StyleParseResult<Vec<Style>> {
    let mut result = Vec::new();
    let mut key = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::Key => {
                key = Some(pair.as_str().to_owned());
            }
            Rule::Value => {
                if let Some(key) = &key {
                    parse_styles_from_value(key, pair.into_inner(), &mut result)?;
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(result)
}

fn parse_styles_from_value(
    key: &str,
    pairs: Pairs<Rule>,
    result: &mut Vec<Style>,
) -> StyleParseResult<()> {
    match key {
        "size" => parse_size_styles(parse_unnamed_block_to_map(pairs)?, result)?,
        "margin" => parse_margin_styles(parse_unnamed_block_to_map(pairs)?, result)?,
        "padding" => parse_padding_styles(parse_unnamed_block_to_map(pairs)?, result)?,
        "font" => parse_font_styles(parse_unnamed_block_to_map(pairs)?, result)?,
        _ => Err(format!("Property with key '{}' is currently not supported", key).to_owned())?,
    }

    Ok(())
}

fn parse_unnamed_block_to_map(pairs: Pairs<Rule>) -> StyleParseResult<HashMap<String, String>> {
    let mut result = HashMap::new();

    for pair in pairs {
        if let Rule::UnnamedBlock = pair.as_rule() {
            for pair in pair.into_inner() {
                if let Rule::KeyValuePair = pair.as_rule() {
                    let mut key = None;
                    let mut value = None;

                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::Key => {
                                key = Some(pair.as_str().to_owned());
                            }
                            Rule::Value => {
                                value = pair.as_str().strip_suffix(";").map(|s| s.to_owned());
                            }
                            _ => unreachable!(),
                        }
                    }

                    if let (Some(key), Some(value)) = (key, value) {
                        result.insert(key, value);
                    }
                }
            }
        }
    }

    Ok(result)
}

fn parse_size_styles(
    properties: HashMap<String, String>,
    result: &mut Vec<Style>,
) -> StyleParseResult<()> {
    if properties.contains_key("width") {
        let distance = parse_distance_property(&properties, "width")?;
        result.push(Style::Width(distance));
    }

    if properties.contains_key("height") {
        let distance = parse_distance_property(&properties, "height")?;
        result.push(Style::Height(distance));
    }

    Ok(())
}

fn parse_margin_styles(
    properties: HashMap<String, String>,
    result: &mut Vec<Style>,
) -> StyleParseResult<()> {
    if properties.contains_key("top") {
        let distance = parse_distance_property(&properties, "top")?;
        result.push(Style::MarginTop(distance));
    }

    if properties.contains_key("left") {
        let distance = parse_distance_property(&properties, "left")?;
        result.push(Style::MarginLeft(distance));
    }

    if properties.contains_key("bottom") {
        let distance = parse_distance_property(&properties, "bottom")?;
        result.push(Style::MarginBottom(distance));
    }

    if properties.contains_key("right") {
        let distance = parse_distance_property(&properties, "right")?;
        result.push(Style::MarginRight(distance));
    }

    Ok(())
}

fn parse_padding_styles(
    properties: HashMap<String, String>,
    result: &mut Vec<Style>,
) -> StyleParseResult<()> {
    if properties.contains_key("top") {
        let distance = parse_distance_property(&properties, "top")?;
        result.push(Style::PaddingTop(distance));
    }

    if properties.contains_key("left") {
        let distance = parse_distance_property(&properties, "left")?;
        result.push(Style::PaddingLeft(distance));
    }

    if properties.contains_key("bottom") {
        let distance = parse_distance_property(&properties, "bottom")?;
        result.push(Style::PaddingBottom(distance));
    }

    if properties.contains_key("right") {
        let distance = parse_distance_property(&properties, "right")?;
        result.push(Style::PaddingRight(distance));
    }

    Ok(())
}

fn parse_font_styles(
    properties: HashMap<String, String>,
    result: &mut Vec<Style>,
) -> StyleParseResult<()> {
    if properties.contains_key("size") {
        let distance = parse_distance_property(&properties, "size")?;
        result.push(Style::FontSize(distance));
    }
    
    if properties.contains_key("family") {
        let family = properties.get("family").unwrap().as_str().trim();
        
        match family {
            "default" => result.push(Style::FontFamily(FontFamilySource::Default)),
            _ => {
                let is_url = family.starts_with("url(") && family.ends_with(")");
                if is_url {
                    let url = family
                        .strip_prefix("url(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .to_owned();
                    result.push(Style::FontFamily(FontFamilySource::Path(url)));
                } else {
                    result.push(Style::FontFamily(FontFamilySource::Name(family.to_owned())));
                }
            }
        };
    }

    Ok(())
}

fn parse_distance_property(
    properties: &HashMap<String, String>,
    key: &str,
) -> StyleParseResult<Distance> {
    if let Some(value) = properties.get(key) {
        let (number_str, _) = value
            .split_once(|c: char| c.is_alphabetic())
            .ok_or("No unit defined for width")?;
        let number = number_str.parse::<f64>()?;
        let unit = value
            .strip_prefix(number_str)
            .ok_or("No unit defined for width")?;
        let unit = DistanceUnit::from_shortform(&unit).ok_or("No unit defined for width")?;

        return Ok(Distance::new(number, unit));
    }

    Err(format!("No value for property '{}' defined", key)
        .to_owned()
        .into())
}

fn parse_selector(pairs: Pairs<Rule>) -> StyleParseResult<Selector> {
    let selectables = pairs
        .map(|pair| parse_selectable(pair.into_inner()))
        .collect::<StyleParseResult<Vec<Selectable>>>()?;

    Ok(Selector { selectables })
}

fn parse_selectable(pairs: Pairs<Rule>) -> StyleParseResult<Selectable> {
    let mut node_name = None;
    let mut class_name = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::NodeName => {
                node_name = Some(pair.as_str().to_owned());
            }
            Rule::ClassName => {
                class_name = Some(
                    pair.as_str()
                        .to_owned()
                        .strip_prefix(".")
                        .unwrap()
                        .to_owned(),
                );
            }
            _ => unreachable!(),
        }
    }

    Ok(Selectable {
        node_name: node_name.ok_or("Node name is required")?,
        class_name,
    })
}

// TODO Extract structs and types into separate files in an parser module

#[derive(Debug)]
struct Selector {
    selectables: Vec<Selectable>,
}

#[derive(Debug)]
struct Selectable {
    node_name: NodeName,
    class_name: Option<ClassName>,
}
