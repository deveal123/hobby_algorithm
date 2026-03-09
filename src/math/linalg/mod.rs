use super::Arithmetic;

#[derive(Clone)]
pub struct Matrix2<T> {
    pub _row: usize,
    pub _col: usize,
    pub _arr: Vec<T>,
}

pub struct StepIterator<'a, T> {
    start: usize,
    end: usize,
    step: usize,
    arr: &'a Vec<T>,
}

impl<T> Iterator for StepIterator<'_, T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let val = self.arr[self.start];
        self.start += self.step;
        Some(val)
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        let _count = self.end - 1 - self.start;
        _count / self.step
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let idx = self.start + self.step * n;
        if idx >= self.end {
            None
        } else {
            Some(self.arr[idx])
        }
    }
}

impl<T> Matrix2<T>
where
    T: Copy + Default,
{
    pub fn new(row: usize, col: usize, arr: Vec<T>) -> Result<Self, String> {
        if row * col != arr.len() {
            Err("Martix init fail : shape does not match with length of array.".to_string())
        } else {
            Ok(Self {
                _row: row,
                _col: col,
                _arr: arr,
            })
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self._row, self._col)
    }

    pub fn rows(&self, idx: usize) -> Result<StepIterator<'_, T>, String> {
        if idx >= self._row {
            Err("Matrix rows : Out of bounds".to_string())
        } else {
            Ok(StepIterator {
                start: idx * self._col,
                end: (idx + 1) * self._col,
                step: 1,
                arr: &self._arr,
            })
        }
    }

    pub fn cols(&self, idx: usize) -> Result<StepIterator<'_, T>, String> {
        if idx >= self._col {
            Err("Matrix cols : Out of bounds".to_string())
        } else {
            Ok(StepIterator {
                start: idx,
                end: self._row * self._col,
                step: self._col,
                arr: &self._arr,
            })
        }
    }

    pub fn transpose(&self) -> Self {
        let mut new_arr = Vec::new();
        new_arr.resize_with(self._row * self._col, T::default);

        self._arr.iter().enumerate().for_each(|(ij, val)| {
            let (i, j) = (ij / self._col, ij % self._col);
            new_arr[j * self._row + i] = *val;
        });
        Self {
            _row: self._col,
            _col: self._row,
            _arr: new_arr,
        }
    }
}

impl<T> std::fmt::Debug for Matrix2<T>
where
    T: std::fmt::Debug + Copy + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self._row {
            write!(f, "[")?;
            let row_iter = self.rows(i).unwrap();
            for (i, val) in row_iter.enumerate() {
                if i == 0 {
                    write!(f, "{:?}", val)?;
                } else {
                    write!(f, " {:?}", val)?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "]")
    }
}

impl<T> std::ops::Add for Matrix2<T>
where
    T: Arithmetic,
{
    type Output = Result<Matrix2<T>, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self._row != rhs._row || self._col != rhs._col {
            Err("Matrix add : Shape Mismatch.".to_string())
        } else {
            let _arr = self
                ._arr
                .iter()
                .zip(rhs._arr.iter())
                .map(|(l, r)| *l + *r)
                .collect::<Vec<_>>();
            Ok(Self {
                _row: self._row,
                _col: self._col,
                _arr,
            })
        }
    }
}

impl<T> std::ops::Sub for Matrix2<T>
where
    T: Arithmetic,
{
    type Output = Result<Matrix2<T>, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self._row != rhs._row || self._col != rhs._col {
            Err("Matrix sub : Shape Mismatch.".to_string())
        } else {
            let _arr = self
                ._arr
                .iter()
                .zip(rhs._arr.iter())
                .map(|(l, r)| *l - *r)
                .collect::<Vec<_>>();
            Ok(Self {
                _row: self._row,
                _col: self._col,
                _arr,
            })
        }
    }
}

impl<T> std::ops::Mul for Matrix2<T>
where
    T: Arithmetic + Default,
{
    type Output = Result<Matrix2<T>, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self._col != rhs._row {
            Err("Matrix mul : Shape Mismatch.".to_string())
        } else {
            let rhs_transposed = rhs.transpose();
            let mut _arr = Vec::with_capacity(self._row * rhs._col);
            for i in 0..self._row {
                for j in 0..rhs._col {
                    _arr.push(
                        self.rows(i)
                            .unwrap()
                            .zip(rhs_transposed.rows(j).unwrap())
                            .map(|(l, r)| l * r)
                            .sum(),
                    );
                }
            }

            Ok(Self {
                _row: self._row,
                _col: rhs._col,
                _arr,
            })
        }
    }
}

impl<T> Matrix2<T>
where
    T: Arithmetic + Default,
{
    pub fn identity(n: usize) -> Self {
        let mut _arr: Vec<T> = Vec::with_capacity(n * n);
        _arr.resize_with(n * n, T::zero);
        (0..n).for_each(|i| {
            _arr[i * (n + 1)] = T::one();
        });
        Self {
            _row: n,
            _col: n,
            _arr,
        }
    }
    pub fn pow(&self, n: usize) -> Result<Self, String> {
        if self._row != self._col {
            return Err("Matrix pow: # of row and # of col are different.".to_string());
        }
        let mut res = Matrix2::<T>::identity(self._row);
        let mut multiplier = self.clone();
        let mut _n = n;
        while _n > 0 {
            if (_n & 1) == 1 {
                res = (res * multiplier.clone())?;
            }
            multiplier = (multiplier.clone() * multiplier)?;
            _n >>= 1;
        }
        Ok(res)
    }
}

impl<T: Arithmetic> std::ops::Index<(usize, usize)> for Matrix2<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self._arr.index(index.0 * self._col + index.1)
    }
}

impl<T: Arithmetic> std::ops::IndexMut<(usize, usize)> for Matrix2<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self._arr.index_mut(index.0 * self._col + index.1)
    }
}
