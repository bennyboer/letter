use result::ExportResult;
use typeset::element::DocumentLayout;

mod export_type;
mod pdf;
mod result;

pub use export_type::ExportType;

pub fn export(document_layout: DocumentLayout, export_type: ExportType) -> ExportResult<()> {
    match export_type {
        ExportType::PDF => pdf::export_as_pdf(document_layout),
    }?;

    Ok(())
}
