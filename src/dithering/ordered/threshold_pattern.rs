pub trait ThresholdPattern: Sync {
    fn at_wrapping(&self, x: u32, y: u32) -> f32;
}
