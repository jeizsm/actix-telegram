use super::*;

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Serialize, Deserialize, Debug)]
pub struct MaskPosition {
    point: String,
    x_shift: Float,
    y_shift: Float,
    scale: Float,
}