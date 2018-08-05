use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MaskPosition {
    point: String,
    x_shift: Float,
    y_shift: Float,
    scale: Float,
}