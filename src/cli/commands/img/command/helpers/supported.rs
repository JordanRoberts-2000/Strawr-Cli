use std::path::Path;

pub fn is_supported_image(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp"),
        None => false,
    }
}
