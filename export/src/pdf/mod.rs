use std::{fs::File, io::BufWriter};

use printpdf::{Mm, PdfDocument};
use typeset::element::Page;
use unit::DistanceUnit;

use crate::result::ExportResult;

pub(crate) fn export_as_pdf(pages: Vec<Page>) -> ExportResult<()> {
    let (document, page_index, layer_index) =
        PdfDocument::new("Letter test output", Mm(210.0), Mm(297.0), "Layer 1");
    let mut is_first_page = true;
    let mut pdf_page = document.get_page(page_index);
    let mut pdf_layer = pdf_page.get_layer(layer_index);

    for page in pages {
        if !is_first_page {
            let (page_index, layer_index) = document.add_page(
                Mm(210.0),
                Mm(297.0),
                format!("Page {}, Layer 1", page.number()),
            );
            pdf_page = document.get_page(page_index);
            pdf_layer = pdf_page.get_layer(layer_index);
        }

        draw_elements_on_layer(&document, &pdf_layer, page.elements());

        is_first_page = false;
    }

    document
        .save(&mut BufWriter::new(File::create("out.pdf").unwrap()))
        .unwrap();

    Ok(())
}

fn draw_elements_on_layer(
    document: &printpdf::PdfDocumentReference, // TODO Probably dont need to pass the document when the fonts are loaded using some kind of font manager
    pdf_layer: &printpdf::PdfLayerReference,
    elements: &[typeset::element::TypesetElement],
) {
    // TODO Loading the font everytime is expensive -> need some kind of font cache to only load a font once
    let font = document
        .add_external_font(File::open("C:/Windows/Fonts/TisaPro.otf").unwrap())
        .unwrap();

    for element in elements {
        let position = element.bounds().position();

        match element.content() {
            typeset::element::TypesetElementContent::Group(content) => {
                draw_elements_on_layer(document, pdf_layer, &content.elements);
            }
            typeset::element::TypesetElementContent::TextSlice(content) => {
                pdf_layer.use_text(
                    &content.text,
                    12.0,
                    Mm(20.0 + position.x().value(DistanceUnit::Millimeter)),
                    Mm(297.0 - 20.0 - position.y().value(DistanceUnit::Millimeter)),
                    &font,
                );
            }
            _ => {}
        };
    }
}
