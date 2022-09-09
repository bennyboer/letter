use result::ExportResult;
use typeset::element::Page;

mod export_type;
mod pdf;
mod result;

pub use export_type::ExportType;

pub fn export(pages: Vec<Page>, export_type: ExportType) -> ExportResult<()> {
    match export_type {
        ExportType::PDF => pdf::export_as_pdf(pages),
    }?;

    Ok(())
}
