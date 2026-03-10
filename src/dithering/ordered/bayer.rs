pub struct BayerMatrix {
    size: usize,
    data: Vec<u32>,
}

#[derive(Debug, thiserror::Error)]
pub enum BayerError {
    #[error("Invalid matrix power: {0}. Power must be at least 1")]
    InvalidSize(usize),
    #[error("Index out of bounds: ({row}, {col}) for matrix of size {size}")]
    IndexOutOfBounds { row: usize, col: usize, size: usize },
    #[error("Failed to allocate matrix of size {0}")]
    AllocationFailed(usize),
}

impl BayerMatrix {}
