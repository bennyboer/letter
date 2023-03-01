use std::io::Cursor;
use std::{fs::File, io::BufWriter};

use printpdf::{Color, Greyscale, Line, Mm, PdfDocument, Point};

use layout::element::content::LayoutElementContent;
use layout::element::{DocumentLayout, Page};
use unit::{Distance, DistanceUnit};

use crate::result::ExportResult;

pub(crate) fn export_as_pdf(document_layout: DocumentLayout) -> ExportResult<()> {
    let layout_constraints = document_layout.pages().first().unwrap().constraints();

    let initial_page_width = Mm(layout_constraints
        .size()
        .width
        .value(DistanceUnit::Millimeter));
    let initial_page_height = Mm(layout_constraints
        .size()
        .height
        .value(DistanceUnit::Millimeter));

    let (document, page_index, layer_index) = PdfDocument::new(
        "Letter test output",
        initial_page_width,
        initial_page_height,
        "Layer 1",
    );
    let mut is_first_page = true;
    let mut pdf_page = document.get_page(page_index);
    let mut pdf_layer = pdf_page.get_layer(layer_index);

    for page in document_layout.pages() {
        if !is_first_page {
            let (page_index, layer_index) = document.add_page(
                initial_page_width,
                initial_page_height,
                format!("Page {}, Layer 1", page.number()),
            );
            pdf_page = document.get_page(page_index);
            pdf_layer = pdf_page.get_layer(layer_index);
        }

        draw_page_content_outline(&pdf_layer, page);
        draw_elements_on_layer(&document, &pdf_layer, &document_layout, page);

        is_first_page = false;
    }

    document
        .save(&mut BufWriter::new(File::create("out.pdf").unwrap()))
        .unwrap();

    Ok(())
}

fn draw_page_content_outline(pdf_layer: &printpdf::PdfLayerReference, page: &Page) {
    let layout_constraints = page.constraints();
    let page_width = layout_constraints
        .size()
        .width
        .value(DistanceUnit::Millimeter);
    let page_height = layout_constraints
        .size()
        .height
        .value(DistanceUnit::Millimeter);

    pdf_layer.set_outline_color(Color::Greyscale(Greyscale::new(0.9, None)));
    let page_content_bounds_line_points = vec![
        (Point::new(Mm(20.0), Mm(20.0)), false),
        (Point::new(Mm(20.0), Mm(page_height - 20.0)), false),
        (
            Point::new(Mm(page_width - 20.0), Mm(page_height - 20.0)),
            false,
        ),
        (Point::new(Mm(page_width - 20.), Mm(20.0)), false),
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
    page: &Page,
) {
    let layout_constraints = page.constraints();
    let page_height = layout_constraints
        .size()
        .height
        .value(DistanceUnit::Millimeter);

    // TODO Add all fonts at the beginning of rendering the PDF instead of here!
    // TODO Subset font before using
    let letter_font = document_layout.get_default_font(); // TODO Implement real font lookup
    let font_data = letter_font.to_internal().face().face_data().to_vec();
    let font_read_cursor = Cursor::new(font_data);
    let font = document.add_external_font(font_read_cursor).unwrap();

    for element_id in page.elements() {
        if let Some(element) = document_layout.element(element_id) {
            let position = element.bounds().position();

            match element.content() {
                LayoutElementContent::TextSlice(content) => {
                    let font_size = content.font_size;

                    pdf_layer.begin_text_section();

                    pdf_layer.set_font(&font, font_size.value(DistanceUnit::Points));
                    pdf_layer.set_text_cursor(
                        Mm(position.x().value(DistanceUnit::Millimeter)),
                        Mm(page_height
                            - (position.y() + font_size).value(DistanceUnit::Millimeter)),
                    );

                    // TODO Find "normal" codepoint width for each glyph for the current font
                    // TODO And calculate the difference between the text shaper result x_advance -> pass difference to write_positioned_codepoints
                    let mut advance_adjustments = Vec::new();
                    let mut next_advance_adjustment = Distance::zero();
                    for glyph_details in &content.glyphs {
                        let advance_adjustment =
                            glyph_details.font_x_advance - glyph_details.x_advance;

                        advance_adjustments.push(next_advance_adjustment);
                        next_advance_adjustment = advance_adjustment;
                    }
                    let converted_advance_adjustments =
                        advance_adjustments.iter().map(|adjustment| {
                            adjustment.value(DistanceUnit::FontUnits {
                                units_per_em: 1000,
                                font_size: font_size.value(DistanceUnit::Millimeter),
                            }) as i64
                        });
                    let codepoints = content
                        .glyphs
                        .iter()
                        .map(|glyph_details| glyph_details.codepoint as u16);

                    pdf_layer
                        .write_positioned_codepoints(converted_advance_adjustments.zip(codepoints));

                    pdf_layer.end_text_section();
                }
                _ => {}
            };
        }
    }
}
