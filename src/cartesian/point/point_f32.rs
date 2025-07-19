#[derive(PartialEq, Debug, Clone)]
pub struct PointF32 {
    pub x: f32,
    pub y: f32,
}

impl PointF32 {
    pub fn of(x: f32, y: f32) -> Self {
        PointF32 { x, y }
    }
}

impl std::fmt::Display for PointF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::PointF32;

    #[test]
    fn point_u8() {
        assert_eq!(PointF32::of(f32::MIN, f32::MAX), PointF32 { x: f32::MIN, y: f32::MAX });
        assert_eq!(
            PointF32::of(f32::MIN, f32::MAX).to_string(),
            "(-340282350000000000000000000000000000000, 340282350000000000000000000000000000000)"
        );
    }
}
