use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Matrix<const N: usize, T> {
    rows: usize,
    cols: usize,
    data: [T; N],
}

impl<const N: usize, T> Matrix<N, T>
where
    T: Copy + Default,
{
    pub const fn new(rows: usize, cols: usize, data: [T; N]) -> Self {
        Self { rows, cols, data }
    }

    pub fn from_fn<F: Fn(usize, usize) -> T>(rows: usize, cols: usize, init: F) -> Self {
        let mut data = [T::default(); N];

        for r in 0..rows {
            for c in 0..cols {
                data[r * cols + c] = init(r, c);
            }
        }

        Self { rows, cols, data }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.rows).map(|r| &self[r])
    }

    pub fn add_row(&mut self) -> &mut [T] {
        let idx = self.rows;
        self.rows += 1;

        &mut self[idx]
    }
}

impl<const N: usize, T> Index<usize> for Matrix<N, T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.cols * index..(index + 1) * self.cols]
    }
}

impl<const N: usize, T> IndexMut<usize> for Matrix<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[self.cols * index..(index + 1) * self.cols]
    }
}

impl<const N: usize, T> std::fmt::Display for Matrix<N, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{:^5}", self.data[r * self.cols + c])?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const N: usize> Matrix<N, f64> {
    pub fn rref(&mut self) {
        let mut i_col = 0;

        for r in 0..self.rows {
            // 1. Find row and column of next pivot entry
            let mut i_row = r;
            while is_zero(&self[i_row][i_col]) {
                i_row += 1;
                if i_row == self.rows {
                    i_row = r;
                    i_col += 1;

                    if i_col == self.cols {
                        return;
                    }
                }
            }

            // 2. Move row up
            for i in 0..self.cols {
                self.data.swap((i_row * self.cols) + i, (r * self.cols) + i);
            }

            // 3. Scale pivot entry to 1
            let mut pivot_value = self[r][i_col];
            for val in self[r].iter_mut() {
                *val /= pivot_value;
            }

            // 4. Subtract values in pivot column to get 0
            for i_row in 0..self.rows {
                if i_row != r {
                    pivot_value = self[i_row][i_col];

                    let [xs, ys] = self
                        .data
                        .get_disjoint_mut([
                            r * self.cols..(r + 1) * self.cols,
                            i_row * self.cols..(i_row + 1) * self.cols,
                        ])
                        .unwrap();

                    for (x, y) in xs.iter_mut().zip(ys.iter_mut()) {
                        *y -= *x * pivot_value;
                    }
                }
            }

            // 5. Move to next column
            i_col += 1;
            if i_col >= self.cols {
                return;
            }
        }
    }

    pub fn bound_variables(&self) -> Vec<Option<f64>> {
        let mut out = vec![None; self.cols - 1];

        for r in 0..self.rows {
            let row = &self[r];
            let Some(pivot_col) = row[0..row.len() - 1]
                .iter()
                .position(|v| is_zero(&(v - 1.)))
            else {
                continue;
            };

            if row[pivot_col + 1..row.len() - 1].iter().all(is_zero) {
                out[pivot_col] = Some(row[row.len() - 1]);
            }
        }

        out
    }
}

fn is_zero(x: &f64) -> bool {
    x.abs() < 0.00001
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_fn() {
        let matrix = Matrix::from_fn(3, 3, |r, c| if r == c { 1 } else { 0 });

        assert_eq!(matrix.data, [1, 0, 0, 0, 1, 0, 0, 0, 1]);
    }

    #[test]
    pub fn test_display() {
        let matrix = Matrix::new(3, 3, [1, 1, 0, 0, 1, 1, 1, 0, 1]);
        let matrix_str = matrix.to_string();
        let matrix_lines = matrix_str.lines().collect::<Vec<_>>();
        assert_eq!(matrix_lines[0], "  1    1    0  ");
        assert_eq!(matrix_lines[1], "  0    1    1  ");
        assert_eq!(matrix_lines[2], "  1    0    1  ");
    }

    #[test]
    pub fn test_rref() {
        let mut matrix = Matrix::new(
            4,
            7,
            [
                0., 0., 0., 0., 1., 1., 3., //
                0., 1., 0., 0., 0., 1., 5., //
                0., 0., 1., 1., 1., 0., 4., //
                1., 1., 0., 1., 0., 0., 7., //
            ],
        );
        matrix.rref();

        assert_eq!(
            matrix.data,
            [
                1., 0., 0., 1., 0., -1., 2., //
                0., 1., 0., 0., 0., 1., 5., //
                0., 0., 1., 1., 0., -1., 1., //
                0., 0., 0., 0., 1., 1., 3., //
            ]
        )
    }
}
