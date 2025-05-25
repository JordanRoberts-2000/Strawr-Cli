mod command;
mod impls {
    mod execute;
    mod handlers {
        mod alt;
        mod blur_data_url;
        mod color;
        mod data_url;
        mod default;
        mod hash;
    }
}

pub use command::GetSubcommmand;
