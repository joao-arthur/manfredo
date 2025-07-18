pub struct RectU8 {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
}

impl RectU8 {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        RectU8 { x1, y1, x2, y2 }
    }
}

pub fn delta_x(r: &RectU8) -> u8 {
    r.x2 - r.x1
}

pub fn delta_y(r: &RectU8) -> u8 {
    r.y2 - r.y1
}

#[cfg(test)]
mod tests {
    use super::{RectU8, delta_x, delta_y};

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU8::of(0, 0, 1, 0)), 1);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, 1)), 1);
    }
}
