use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer};

use crate::ai::AiImageModel;

pub fn dalle_version_parse(s: &str) -> Result<AiImageModel, String> {
    match s {
        "2" | "dalle2" => Ok(AiImageModel::Dalle2),
        "3" | "dalle3" => Ok(AiImageModel::Dalle3),
        other => Err(format!(
            "invalid DALL-E version `{}`, expected 2 or 3",
            other
        )),
    }
}

pub fn dalle_version_deserialize<'de, D>(de: D) -> Result<AiImageModel, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u8::deserialize(de)?;
    match v {
        2 => Ok(AiImageModel::Dalle2),
        3 => Ok(AiImageModel::Dalle3),
        other => Err(DeError::custom(format!(
            "invalid DALL-E version `{}`, expected 2 or 3",
            other
        ))),
    }
}
