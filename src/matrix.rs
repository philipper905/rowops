#[derive(Debug)]
pub struct Matrix {
    rows: u8,
    cols: u8,
    inner: Vec<f64>
    pub rows: u8,
    pub cols: u8,
}

impl Matrix {
    pub fn new(rows: u8, cols: u8) -> Matrix{
        Matrix {
            rows,
            cols,
            inner: vec![0.0; (rows * cols) as usize],
        } 
    }
}

#[derive(Default)]
pub struct MatrixBuilder {
    rows: u8,
    cols: u8,
}

impl MatrixBuilder {
    pub fn new() -> MatrixBuilder {
        MatrixBuilder::default()
    }

    pub fn rows(mut self, rows: u8) -> MatrixBuilder {
        self.rows = rows;
        return self;
    }

    pub fn columns(mut self, columns: u8) -> MatrixBuilder {
        self.cols = columns;
        return self;
    }

    pub fn cols(self, cols: u8) -> MatrixBuilder {
        self.columns(cols)
    }

    pub fn build(self) -> Matrix {
        Matrix::new(self.rows, self.cols)
    }
}

impl From<MatrixBuilder> for Matrix {
    fn from(x: MatrixBuilder) -> Matrix {
        x.build()
    }
}
