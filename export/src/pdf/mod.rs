use std::{fs::File, io::BufWriter};

use printpdf::{Mm, PdfDocument};
use typeset::element::Page;

use crate::result::ExportResult;

pub(crate) fn export_as_pdf(_pages: Vec<Page>) -> ExportResult<()> {
    let (doc, _page1, _layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    doc.add_page(Mm(210.0), Mm(297.0), "Page 2, Layer 1");

    doc.save(&mut BufWriter::new(File::create("out.pdf").unwrap()))
        .unwrap();

    Ok(())
}
