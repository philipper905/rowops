#[derive(Debug)]
pub struct Matrix {
    pub rows: u8,
    pub cols: u8,
    inner: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: u8, cols: u8) -> Matrix {
        Matrix {
            rows,
            cols,
            inner: vec![vec![0.0; cols as usize]; rows as usize],
        }
    }

    pub fn get(&self, row: u8, col: u8) -> &f64 {
        assert!(row < self.rows);
        assert!(col < self.cols);

        &self.inner[row as usize][col as usize]
    }

    pub fn set(&mut self, row: u8, col: u8, val: f64) {
        *self.get_mut(row, col) = val;
    }

    pub fn get_mut(&mut self, row: u8, col: u8) -> &mut f64 {
        assert!(row < self.rows);
        assert!(col < self.cols);

        &mut self.inner[row as usize][col as usize]
    }

    pub fn get_row(&self, row: u8) -> &[f64] {
        assert!(row < self.rows);

        &self.inner[row as usize]
    }

    pub fn get_row_mut(&mut self, row: u8) -> &mut [f64] {
        assert!(row < self.rows);

        &mut self.inner[row as usize]
    }

    fn get_two_row_mut(&mut self, row_a: u8, row_b: u8) -> (&mut [f64], &mut [f64]) {
        assert!(row_a < self.rows);
        assert!(row_b < self.rows);
        assert!(row_a != row_b);

        let smaller = std::cmp::min(row_a, row_b) as usize;
        let larger = std::cmp::max(row_a, row_b) as usize;

        let (first_half, second_half) = self.inner.split_at_mut(smaller + 1);
        let row_a = &mut first_half[smaller];
        let row_b = &mut second_half[larger - smaller];

        (row_a, row_b)
    }


    pub fn scale(&mut self, row: u8, scalar: f64) {
        assert!(row < self.rows);

        self.get_row_mut(row).iter_mut().for_each(|elem| {
            *elem *= scalar;
        });
    }

    pub fn swap(&mut self, row_a: u8, row_b: u8) {
        assert!(row_a < self.rows);
        assert!(row_b < self.rows);
        assert!(row_a != row_b);

        self.inner.swap(row_a as usize, row_b as usize);
    }

    pub fn add_onto(&mut self, add_row: u8, onto: u8, by_factor: f64) {
        assert!(add_row < self.rows);
        assert!(onto < self.rows);
        assert!(onto != add_row);

        let (row_a, row_b) = self.get_two_row_mut(add_row, onto);
        let (add_row, onto) = {
            if add_row < onto {
                (row_a, row_b)
            } else {
                (row_b, row_a)
            }
        };

        onto.iter_mut()
            .zip(add_row.iter().map(|elem| elem * by_factor))
            .for_each(|(onto_elem, add_row_elem)| {
                *onto_elem += add_row_elem;
            })
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
