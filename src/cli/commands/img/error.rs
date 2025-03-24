use crate::services::img;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImgError {
    #[error("Internal error occured: {0}")]
    ImgFailed(img::ImgError),
}
