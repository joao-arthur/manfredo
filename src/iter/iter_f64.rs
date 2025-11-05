#[derive(Debug, PartialEq)]
pub struct Iter {
    start: f64,
    end: f64,
}

impl Iter {
    pub fn new(start: f64, end: f64) -> Self {
        Iter { start, end }
    }
}

impl std::fmt::Display for Iter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..={}", self.start, self.end)
    }
}

impl Iterator for Iter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }
        let result = self.start;
        self.start += 1.0;
        Some(result)
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }
        let result = self.end;
        self.end -= 1.0;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Iter;

    #[test]
    fn iter() {
        assert_eq!(Iter::new(-10.0, 10.0), Iter { start: -10.0, end: 10.0 });
        assert_eq!(Iter::new(-10.0, 10.0).to_string(), "-10..=10");
    }

    #[test]
    fn iterator_ending_in_0() {
        assert_eq!(Iter::new(-9.0, -10.0).collect::<Vec<f64>>(), []);
        assert_eq!(Iter::new(-9.0, -9.0).collect::<Vec<f64>>(), [-9.0]);
        assert_eq!(Iter::new(-9.0, -8.0).collect::<Vec<f64>>(), [-9.0, -8.0]);
        assert_eq!(Iter::new(-9.0, -7.0).collect::<Vec<f64>>(), [-9.0, -8.0, -7.0]);
        assert_eq!(Iter::new(-9.0, -6.0).collect::<Vec<f64>>(), [-9.0, -8.0, -7.0, -6.0]);
    }

    #[test]
    fn iterator_reversed() {
        assert_eq!(Iter::new(-9.0, -10.0).rev().collect::<Vec<f64>>(), []);
        assert_eq!(Iter::new(-9.0, -9.0).rev().collect::<Vec<f64>>(), [-9.0]);
        assert_eq!(Iter::new(-9.0, -8.0).rev().collect::<Vec<f64>>(), [-8.0, -9.0]);
        assert_eq!(Iter::new(-9.0, -7.0).rev().collect::<Vec<f64>>(), [-7.0, -8.0, -9.0]);
        assert_eq!(Iter::new(-9.0, -6.0).rev().collect::<Vec<f64>>(), [-6.0, -7.0, -8.0, -9.0]);
    }

    #[test]
    fn iterator_ending_in_5() {
        assert_eq!(Iter::new(-9.5, -10.5).collect::<Vec<f64>>(), []);
        assert_eq!(Iter::new(-9.5, -9.5).collect::<Vec<f64>>(), [-9.5]);
        assert_eq!(Iter::new(-9.5, -8.5).collect::<Vec<f64>>(), [-9.5, -8.5]);
        assert_eq!(Iter::new(-9.5, -7.5).collect::<Vec<f64>>(), [-9.5, -8.5, -7.5]);
        assert_eq!(Iter::new(-9.5, -6.5).collect::<Vec<f64>>(), [-9.5, -8.5, -7.5, -6.5]);
    }

    #[test]
    fn iterator_ending_in_25() {
        assert_eq!(Iter::new(-9.25, -10.25).collect::<Vec<f64>>(), []);
        assert_eq!(Iter::new(-9.25, -9.25).collect::<Vec<f64>>(), [-9.25]);
        assert_eq!(Iter::new(-9.25, -8.25).collect::<Vec<f64>>(), [-9.25, -8.25]);
        assert_eq!(Iter::new(-9.25, -7.25).collect::<Vec<f64>>(), [-9.25, -8.25, -7.25]);
        assert_eq!(Iter::new(-9.25, -6.25).collect::<Vec<f64>>(), [-9.25, -8.25, -7.25, -6.25]);
    }

    #[test]
    fn iterator_ending_in_75() {
        assert_eq!(Iter::new(-9.75, -10.75).collect::<Vec<f64>>(), []);
        assert_eq!(Iter::new(-9.75, -9.75).collect::<Vec<f64>>(), [-9.75]);
        assert_eq!(Iter::new(-9.75, -8.75).collect::<Vec<f64>>(), [-9.75, -8.75]);
        assert_eq!(Iter::new(-9.75, -7.75).collect::<Vec<f64>>(), [-9.75, -8.75, -7.75]);
        assert_eq!(Iter::new(-9.75, -6.75).collect::<Vec<f64>>(), [-9.75, -8.75, -7.75, -6.75]);
    }

    #[test]
    fn iterator_ending_not_matching() {
        assert_eq!(Iter::new(1.0, 4.1).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.2).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.3).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.4).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.5).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.6).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.7).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.8).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 4.9).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(Iter::new(1.0, 5.0).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.1).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.2).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.3).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.4).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.5).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.6).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.7).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.8).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(Iter::new(1.0, 5.9).collect::<Vec<f64>>(), [1.0, 2.0, 3.0, 4.0, 5.0]);
    }
}
