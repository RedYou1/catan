#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub min: f32,
    pub max: Option<f32>,
}

impl Range {
    pub fn fix_sized(&self) -> bool {
        if let Some(max) = self.max {
            self.min - f32::EPSILON <= max &&
            self.min + f32::EPSILON >= max
        } else {
            false
        }
    }
}
