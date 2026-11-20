//! Experimental in-memory mockup adapter. No filesystem or UI is required.
//! This does not yet implement fit/fill or claim Photoshop pixel parity.

use photocraft_doc::LayerContent;
use photocraft_engine::{smart_cmds, Session};
use photocraft_io::ExportOptions;

#[derive(Debug, PartialEq, Eq)]
pub enum MockupError {
    Import(String),
    LayerNotFound(String),
    AmbiguousLayer(String),
    NotSmartObject(String),
    Replace(String),
    Export(String),
}

pub fn render_mockup_png(
    template_name: &str,
    template_bytes: &[u8],
    layer_name: &str,
    design_name: &str,
    design_bytes: &[u8],
) -> Result<(Vec<u8>, Vec<String>), MockupError> {
    let imported = photocraft_io::import(template_name, template_bytes)
        .map_err(|e| MockupError::Import(e.to_string()))?;
    let mut session = Session::new();
    session.add_document(imported.document, None);
    let id = {
        let doc = &session.active().expect("document was just added").doc;
        let mut matches = doc.walk().into_iter()
            .filter(|(_, _, layer)| layer.name == layer_name);
        let first = matches.next()
            .ok_or_else(|| MockupError::LayerNotFound(layer_name.into()))?;
        if matches.next().is_some() {
            return Err(MockupError::AmbiguousLayer(layer_name.into()));
        }
        if !matches!(first.2.content, LayerContent::Smart(_)) {
            return Err(MockupError::NotSmartObject(layer_name.into()));
        }
        first.2.id
    };
    smart_cmds::replace_contents_bytes(&mut session, id, design_name, design_bytes.to_vec())
        .map_err(|e| MockupError::Replace(e.to_string()))?;
    let doc = &session.active().expect("document remains active").doc;
    let exported = photocraft_io::export(doc, "png", &ExportOptions::default())
        .map_err(|e| MockupError::Export(e.to_string()))?;
    Ok((exported.bytes, exported.warnings))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_template_is_rejected() {
        let result = render_mockup_png("invalid.psd", b"not a psd", "Design Here", "logo.png", b"not png");
        assert!(matches!(result, Err(MockupError::Import(_))));
    }
}
