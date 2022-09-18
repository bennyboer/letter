use std::{fs::File, io::BufWriter};

use printpdf::{Color, Greyscale, Line, Mm, PdfDocument, Point};
use typeset::element::{DocumentLayout, ElementId, Page, Position};
use unit::{Distance, DistanceUnit};

use crate::result::ExportResult;

pub(crate) fn export_as_pdf(document_layout: DocumentLayout) -> ExportResult<()> {
    let (document, page_index, layer_index) =
        PdfDocument::new("Letter test output", Mm(210.0), Mm(297.0), "Layer 1");
    let mut is_first_page = true;
    let mut pdf_page = document.get_page(page_index);
    let mut pdf_layer = pdf_page.get_layer(layer_index);

    for page in document_layout.pages() {
        if !is_first_page {
            let (page_index, layer_index) = document.add_page(
                Mm(210.0),
                Mm(297.0),
                format!("Page {}, Layer 1", page.number()),
            );
            pdf_page = document.get_page(page_index);
            pdf_layer = pdf_page.get_layer(layer_index);
        }

        draw_page_content_outline(&pdf_layer, page);
        draw_elements_on_layer(&document, &pdf_layer, &document_layout, page.elements());

        is_first_page = false;
    }

    document
        .save(&mut BufWriter::new(File::create("out.pdf").unwrap()))
        .unwrap();

    Ok(())
}

fn draw_page_content_outline(pdf_layer: &printpdf::PdfLayerReference, _page: &Page) {
    // TODO Get outline coordinates from page
    pdf_layer.set_outline_color(Color::Greyscale(Greyscale::new(0.9, None)));
    let page_content_bounds_line_points = vec![
        (Point::new(Mm(20.0), Mm(20.0)), false),
        (Point::new(Mm(20.0), Mm(277.0)), false),
        (Point::new(Mm(190.0), Mm(277.0)), false),
        (Point::new(Mm(190.0), Mm(20.0)), false),
    ];
    let page_content_bounds_line = Line {
        points: page_content_bounds_line_points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    pdf_layer.add_shape(page_content_bounds_line);
}

fn draw_elements_on_layer(
    document: &printpdf::PdfDocumentReference, // TODO Probably dont need to pass the document when the fonts are loaded using some kind of font manager
    pdf_layer: &printpdf::PdfLayerReference,
    document_layout: &DocumentLayout,
    elements: &[ElementId],
) {
    // TODO Loading the font everytime is expensive -> need some kind of font cache to only load a font once
    let font = document
        .add_external_font(File::open("C:/Windows/Fonts/TisaPro.otf").unwrap())
        .unwrap();

    for element_id in elements {
        if let Some(element) = document_layout.element(element_id) {
            let position = resolve_position(element.bounds().position(), document_layout);
            let font_size = Distance::new(12.0, DistanceUnit::Points); // TODO Get from typeset element style

            match element.content() {
                typeset::element::TypesetElementContent::Group(content) => {
                    draw_elements_on_layer(document, pdf_layer, document_layout, &content.elements);
                }
                typeset::element::TypesetElementContent::TextSlice(content) => {
                    pdf_layer.use_text(
                        &content.text,
                        font_size.value(DistanceUnit::Points),
                        Mm(position.x().value(DistanceUnit::Millimeter)),
                        Mm(297.0 - (position.y() + font_size).value(DistanceUnit::Millimeter)), // TODO Use page constraints to get total page height of 297.0mm
                        &font,
                    );
                }
                _ => {}
            };
        }
    }
}

fn resolve_position(position: &Position, document_layout: &DocumentLayout) -> Position {
    if let Position::Absolute { x, y } = position {
        return Position::absolute(*x, *y);
    }

    let mut total_x = Distance::zero();
    let mut total_y = Distance::zero();
    let mut current = position;
    loop {
        total_x += current.x();
        total_y += current.y();

        // TODO Cache already resolved positions for element IDs so that we do not have to loop a lot here
        if let Position::Relative {
            element_id,
            x: _,
            y: _,
        } = current
        {
            current = document_layout
                .element(element_id)
                .expect("Expected the element to exist")
                .bounds()
                .position();
        } else {
            break;
        }
    }

    Position::absolute(total_x, total_y)
}
