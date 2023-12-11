#[derive(Debug)]
pub struct Matrix<T>(Vec<Vec<T>>);

impl<T> Matrix<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y).map(|v| v.get(x)).unwrap_or(None)
    }
}

impl<T: PartialEq> Matrix<T> {
    pub fn find_first(&self, el: &T) -> Option<(usize, usize)> {
        for (y, v) in self.0.iter().enumerate() {
            for (x, e) in v.iter().enumerate() {
                if e == el {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.0 {
            for t in v {
                match write!(f, "{t}") {
                    Ok(_) => {}
                    err => return err,
                }
            }
            match writeln!(f) {
                Ok(_) => {}
                err => return err,
            }
        }
        Ok(())
    }
}

impl<'a, T> Matrix<T> {
    pub fn iter(&'a self) -> MatIter<'a, T> {
        MatIter {
            i: 0,
            j: 0,
            m: self,
        }
    }
}

#[derive(Debug)]
pub struct MatIter<'a, T> {
    i: usize,
    j: usize,
    m: &'a Matrix<T>,
}

impl<'a, T> Iterator for MatIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.m.0.len() <= self.j {
            return None;
        }
        if let Some(v) = self.m.0.get(self.j) {
            if let Some(t) = v.get(self.i) {
                if (self.i + 1) >= v.len() {
                    self.i = 0;
                    self.j += 1;
                } else {
                    self.i += 1;
                }
                return Some(t);
            }
            return None;
        }
        None
    }
}
