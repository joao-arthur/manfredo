use crate::cartesian::d2::{
    point::point_f32,
    rect::rect_f32::{
        Rect,
        delta::{delta_x, delta_y},
    },
};

pub fn len_x(r: &Rect) -> f32 {
    delta_x(r) + 1.0
}

pub fn len_y(r: &Rect) -> f32 {
    delta_y(r) + 1.0
}

pub fn max_len(r: &Rect) -> f32 {
    len_x(r).max(len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{len_x, len_y, max_len};
    use crate::cartesian::d2::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::Rect,
    };

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&Rect::of(0.0, 0.0, 0.0, MAX)), 1.0);
        assert_eq!(len_x(&Rect::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 1.0);
        assert_eq!(len_x(&Rect::of(0.0, 0.0, MAX - 1.0, 0.0)), MAX);
        assert_eq!(len_x(&Rect::of(-8_388_608.0, 0.0, 8_388_606.0, 0.0)), MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&Rect::of(0.0, 0.0, MAX, 0.0)), 1.0);
        assert_eq!(len_y(&Rect::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 1.0);
        assert_eq!(len_y(&Rect::of(0.0, 0.0, 0.0, MAX - 1.0)), MAX);
        assert_eq!(len_y(&Rect::of(0.0, -8_388_608.0, 0.0, 8_388_606.0)), MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&Rect::of(0.0, 5.0, 10.0, 10.0)), 11.0);
        assert_eq!(max_len(&Rect::of(-10.0, -10.0, -5.0, 0.0)), 11.0);
        assert_eq!(max_len(&Rect::of(-5.0, 0.0, 5.0, 5.0)), 11.0);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&Rect::of(0.0, 0.0, 0.0, 0.0)), 1.0);
        assert_eq!(max_len(&Rect::of(1.0, 1.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_len(&Rect::of(-1.0, -1.0, -1.0, -1.0)), 1.0);
        assert_eq!(max_len(&Rect::of(5.0, 10.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&Rect::of(0.0, 0.0, 1.0, 1.0)), 2.0);
        assert_eq!(max_len(&Rect::of(5.0, 5.0, 6.0, 6.0)), 2.0);
        assert_eq!(max_len(&Rect::of(-6.0, -6.0, -5.0, -5.0)), 2.0);
        assert_eq!(max_len(&Rect::of(0.0, 0.0, 0.0, 1.0)), 2.0);
        assert_eq!(max_len(&Rect::of(5.0, 9.0, 5.0, 10.0)), 2.0);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&Rect::of(MIN + 2.0, MIN + 3.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&Rect::of(MIN + 3.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&Rect::of(0.0, 0.0, MAX - 2.0, MAX - 1.0)), MAX);
        assert_eq!(max_len(&Rect::of(0.0, 0.0, MAX - 1.0, MAX - 2.0)), MAX);
    }
}
