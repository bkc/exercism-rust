pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // yields every other item in iterator
    iter.enumerate().filter_map(|(idx, val)| -> Option<T> {
        if 0 == idx % 2 {
            Some(val)
        } else {
            None
        }
    })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
