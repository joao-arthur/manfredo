type Cartesian = crate::cartesian::point::point_i32::Point;
type Matrix = crate::matrix::point::point_u32::Point;
type Cam = crate::matrix::rect::rect_u32::Rect;

pub fn matrix_in_cam_to_cartesian(point: &Matrix, cam: &Cam) -> Cartesian {
    let x = i64::from(i32::MIN) + i64::from(point.col) - i64::from(cam.min.col);
    let y = i64::from(i32::MIN) + i64::from(cam.max.row) - i64::from(point.row);
    Cartesian { x: x as i32, y: y as i32 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, matrix_in_cam_to_cartesian};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 0), &cam), Cartesian::of(i32::MIN, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 1), &cam), Cartesian::of(i32::MIN + 1, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 2), &cam), Cartesian::of(i32::MIN + 2, i32::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 0), &cam), Cartesian::of(i32::MIN, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 1), &cam), Cartesian::of(i32::MIN + 1, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 2), &cam), Cartesian::of(i32::MIN + 2, i32::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 1), &cam), Cartesian::of(i32::MIN + 1, i32::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 2), &cam), Cartesian::of(i32::MIN + 2, i32::MIN));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 10), &cam), Cartesian::of(i32::MIN, i32::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 11), &cam), Cartesian::of(i32::MIN + 1, i32::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 12), &cam), Cartesian::of(i32::MIN + 2, i32::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 13), &cam), Cartesian::of(i32::MIN + 3, i32::MIN + 3));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 10), &cam), Cartesian::of(i32::MIN, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 11), &cam), Cartesian::of(i32::MIN + 1, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 12), &cam), Cartesian::of(i32::MIN + 2, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 13), &cam), Cartesian::of(i32::MIN + 3, i32::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 10), &cam), Cartesian::of(i32::MIN, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 11), &cam), Cartesian::of(i32::MIN + 1, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 12), &cam), Cartesian::of(i32::MIN + 2, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 13), &cam), Cartesian::of(i32::MIN + 3, i32::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 10), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 11), &cam), Cartesian::of(i32::MIN + 1, i32::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 12), &cam), Cartesian::of(i32::MIN + 2, i32::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 13), &cam), Cartesian::of(i32::MIN + 3, i32::MIN));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 0), &cam), Cartesian::of(i32::MIN, i32::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, u32::MAX), &cam), Cartesian::max());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::max(), &cam), Cartesian::of(i32::MAX, i32::MIN));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 1), &cam), Cartesian::of(i32::MIN + 1, i32::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 2), &cam), Cartesian::of(i32::MIN + 2, i32::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(3, 3), &cam), Cartesian::of(i32::MIN + 3, i32::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX - 1, u32::MAX - 1), &cam), Cartesian::of(i32::MAX - 1, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX - 2, u32::MAX - 2), &cam), Cartesian::of(i32::MAX - 2, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX - 3, u32::MAX - 3), &cam), Cartesian::of(i32::MAX - 3, i32::MIN + 3));
    }
}
