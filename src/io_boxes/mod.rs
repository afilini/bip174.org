mod input_boxes;
mod output_boxes;

pub use input_boxes::*;
pub use output_boxes::*;

#[derive(Debug, Clone)]
pub enum IOBoxMsg {
    ToggleExpand(bool),
}
