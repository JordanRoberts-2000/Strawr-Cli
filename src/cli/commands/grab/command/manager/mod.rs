use std::{collections::HashMap, path::PathBuf};

use crate::state::AppContext;

pub mod entry;
pub mod storage;

pub struct GrabManager<'a> {
    pub ctx: &'a AppContext,
    pub json_file_path: PathBuf,
    pub data_map: HashMap<String, String>,
}

impl<'a> GrabManager<'a> {
    pub fn new(ctx: &'a AppContext) -> Self {
        Self {
            ctx,
            json_file_path: PathBuf::new(),
            data_map: HashMap::new(),
        }
    }
}
