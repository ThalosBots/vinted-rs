use crate::model::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Photo {
    id: i64,
    url: String,
    dominant_color: String,
    dominant_color_opaque: String,
}
