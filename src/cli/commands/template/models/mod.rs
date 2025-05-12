mod template {
    pub mod model;
}
mod variant {
    pub mod model;
}

pub use {template::model::Template, variant::model::Variant};
