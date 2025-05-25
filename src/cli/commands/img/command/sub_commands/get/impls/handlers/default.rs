use crate::{commands::img::command::sub_commands::get::GetSubcommmand, img::Img};

impl GetSubcommmand {
    pub fn handle_default(&self, img: &mut Img) {
        println!(
            "{:?} - {}x{} - {}",
            img.format,
            img.width,
            img.height,
            format_size(img.size_bytes)
        );
    }
}

fn format_size(size_bytes: usize) -> String {
    if size_bytes < 1024 {
        format!("{}Bytes", size_bytes)
    } else if size_bytes < 1024 * 1024 {
        format!("{:.1}Kb", size_bytes as f64 / 1024.0)
    } else {
        format!("{:.1}Mb", size_bytes as f64 / (1024.0 * 1024.0))
    }
}
