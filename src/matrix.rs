#[derive(Debug)]
pub struct Matrix {
    rows: u8,
    cols: u8,
    inner: Vec<f64>
}

impl Matrix {
    fn new(rows: u8, cols: u8) -> Matrix{
        Matrix {
            rows,
            cols,
            inner: vec![0.0; (rows * cols) as usize],
        } 
    }
}
