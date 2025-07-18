#[derive(PartialEq, Debug, Clone)]
pub struct PointU8 {
    pub x: u8,
    pub y: u8,
}

impl PointU8 {
    fn from(x: u8, y: u8) -> Self {
        PointU8 { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::PointU8;

    #[test]
    fn point_u8() {
        assert_eq!(PointU8::from(0, 255), PointU8 { x: 0, y: 255 });
    }
}
