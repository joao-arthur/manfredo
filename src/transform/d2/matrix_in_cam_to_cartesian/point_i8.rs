type Cartesian = crate::cartesian::d2::point::point_i8::Point;
type Matrix = crate::matrix::d2::point::point_u8::Point;
type Cam = crate::matrix::d2::rect::rect_u8::Rect;

pub fn matrix_in_cam_to_cartesian(point: &Matrix, cam: &Cam) -> Cartesian {
    let x = i16::from(i8::MIN) + i16::from(point.col) - i16::from(cam.min.col);
    let y = i16::from(i8::MIN) + i16::from(cam.max.row) - i16::from(point.row);
    Cartesian { x: x as i8, y: y as i8 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, matrix_in_cam_to_cartesian};

    #[test]
    fn test_3x3() {
        let cam = Cam::new((0, 0), (2, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::min(), &cam), Cartesian::new(i8::MIN, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, 1), &cam), Cartesian::new(i8::MIN + 1, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, 2), &cam), Cartesian::new(i8::MIN + 2, i8::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 0), &cam), Cartesian::new(i8::MIN, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 1), &cam), Cartesian::new(i8::MIN + 1, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 2), &cam), Cartesian::new(i8::MIN + 2, i8::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 1), &cam), Cartesian::new(i8::MIN + 1, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 2), &cam), Cartesian::new(i8::MIN + 2, i8::MIN));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::new((10, 10), (13, 13));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 10), &cam), Cartesian::new(i8::MIN, i8::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 11), &cam), Cartesian::new(i8::MIN + 1, i8::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 12), &cam), Cartesian::new(i8::MIN + 2, i8::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 13), &cam), Cartesian::new(i8::MIN + 3, i8::MIN + 3));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 10), &cam), Cartesian::new(i8::MIN, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 11), &cam), Cartesian::new(i8::MIN + 1, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 12), &cam), Cartesian::new(i8::MIN + 2, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 13), &cam), Cartesian::new(i8::MIN + 3, i8::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 10), &cam), Cartesian::new(i8::MIN, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 11), &cam), Cartesian::new(i8::MIN + 1, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 12), &cam), Cartesian::new(i8::MIN + 2, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 13), &cam), Cartesian::new(i8::MIN + 3, i8::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 10), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 11), &cam), Cartesian::new(i8::MIN + 1, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 12), &cam), Cartesian::new(i8::MIN + 2, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 13), &cam), Cartesian::new(i8::MIN + 3, i8::MIN));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::min(), &cam), Cartesian::new(i8::MIN, i8::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u8::MAX, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, u8::MAX), &cam), Cartesian::max());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::max(), &cam), Cartesian::new(i8::MAX, i8::MIN));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 1), &cam), Cartesian::new(i8::MIN + 1, i8::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 2), &cam), Cartesian::new(i8::MIN + 2, i8::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(3, 3), &cam), Cartesian::new(i8::MIN + 3, i8::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u8::MAX - 1, u8::MAX - 1), &cam), Cartesian::new(i8::MAX - 1, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u8::MAX - 2, u8::MAX - 2), &cam), Cartesian::new(i8::MAX - 2, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u8::MAX - 3, u8::MAX - 3), &cam), Cartesian::new(i8::MAX - 3, i8::MIN + 3));
    }
}
