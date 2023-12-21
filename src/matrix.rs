#[derive(Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Eq for Matrix<T> {}

impl<T> Matrix<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y).map(|v| v.get(x)).unwrap_or(None)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.0.get_mut(y).map(|v| v.get_mut(x)).unwrap_or(None)
    }

    pub fn pget(&self, p: &Point) -> Option<&T> {
        self.0.get(p.y).map(|v| v.get(p.x)).unwrap_or(None)
    }

    pub fn pget_mut(&mut self, p: &Point) -> Option<&mut T> {
        self.0.get_mut(p.y).map(|v| v.get_mut(p.x)).unwrap_or(None)
    }

    pub fn lenx(&self) -> usize {
        if let Some(v) = self.0.get(0) {
            v.len()
        } else {
            0
        }
    }

    pub fn leny(&self) -> usize {
        self.0.len()
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

    pub fn find_first_p(&self, el: &T) -> Option<Point> {
        for (y, v) in self.0.iter().enumerate() {
            for (x, e) in v.iter().enumerate() {
                if e == el {
                    return Some(Point::new(x, y));
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
