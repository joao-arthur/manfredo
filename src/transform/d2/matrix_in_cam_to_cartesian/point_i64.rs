type Cartesian = crate::cartesian::d2::point::point_i64::Point;
type Matrix = crate::matrix::d2::point::point_u64::Point;
type Cam = crate::matrix::d2::rect::rect_u64::Rect;

pub fn matrix_in_cam_to_cartesian(point: &Matrix, cam: &Cam) -> Cartesian {
    let x = i128::from(i64::MIN) + i128::from(point.col) - i128::from(cam.min.col);
    let y = i128::from(i64::MIN) + i128::from(cam.max.row) - i128::from(point.row);
    Cartesian { x: x as i64, y: y as i64 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, matrix_in_cam_to_cartesian};

    #[test]
    fn test_3x3() {
        let cam = Cam::new((0, 0), (2, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::min(), &cam), Cartesian::new(i64::MIN, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, 1), &cam), Cartesian::new(i64::MIN + 1, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, 2), &cam), Cartesian::new(i64::MIN + 2, i64::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 0), &cam), Cartesian::new(i64::MIN, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 1), &cam), Cartesian::new(i64::MIN + 1, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 2), &cam), Cartesian::new(i64::MIN + 2, i64::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 1), &cam), Cartesian::new(i64::MIN + 1, i64::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 2), &cam), Cartesian::new(i64::MIN + 2, i64::MIN));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::new((10, 10), (13, 13));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 10), &cam), Cartesian::new(i64::MIN, i64::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 11), &cam), Cartesian::new(i64::MIN + 1, i64::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 12), &cam), Cartesian::new(i64::MIN + 2, i64::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 13), &cam), Cartesian::new(i64::MIN + 3, i64::MIN + 3));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 10), &cam), Cartesian::new(i64::MIN, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 11), &cam), Cartesian::new(i64::MIN + 1, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 12), &cam), Cartesian::new(i64::MIN + 2, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 13), &cam), Cartesian::new(i64::MIN + 3, i64::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 10), &cam), Cartesian::new(i64::MIN, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 11), &cam), Cartesian::new(i64::MIN + 1, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 12), &cam), Cartesian::new(i64::MIN + 2, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 13), &cam), Cartesian::new(i64::MIN + 3, i64::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 10), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 11), &cam), Cartesian::new(i64::MIN + 1, i64::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 12), &cam), Cartesian::new(i64::MIN + 2, i64::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 13), &cam), Cartesian::new(i64::MIN + 3, i64::MIN));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::min(), &cam), Cartesian::new(i64::MIN, i64::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u64::MAX, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, u64::MAX), &cam), Cartesian::max());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::max(), &cam), Cartesian::new(i64::MAX, i64::MIN));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 1), &cam), Cartesian::new(i64::MIN + 1, i64::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 2), &cam), Cartesian::new(i64::MIN + 2, i64::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(3, 3), &cam), Cartesian::new(i64::MIN + 3, i64::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u64::MAX - 1, u64::MAX - 1), &cam), Cartesian::new(i64::MAX - 1, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u64::MAX - 2, u64::MAX - 2), &cam), Cartesian::new(i64::MAX - 2, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u64::MAX - 3, u64::MAX - 3), &cam), Cartesian::new(i64::MAX - 3, i64::MIN + 3));
    }
}
