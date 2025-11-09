type Cartesian = crate::cartesian::d2::point::point_i16::Point;
type Matrix = crate::matrix::d2::point::point_u16::Point;
type Cam = crate::matrix::d2::rect::rect_u16::Rect;

pub fn matrix_in_cam_to_cartesian(point: &Matrix, cam: &Cam) -> Cartesian {
    let x = i32::from(i16::MIN) + i32::from(point.col) - i32::from(cam.min.col);
    let y = i32::from(i16::MIN) + i32::from(cam.max.row) - i32::from(point.row);
    Cartesian { x: x as i16, y: y as i16 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, matrix_in_cam_to_cartesian};

    #[test]
    fn test_3x3() {
        let cam = Cam::new((0, 0), (2, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::min(), &cam), Cartesian::new(i16::MIN, i16::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, 1), &cam), Cartesian::new(i16::MIN + 1, i16::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, 2), &cam), Cartesian::new(i16::MIN + 2, i16::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 0), &cam), Cartesian::new(i16::MIN, i16::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 1), &cam), Cartesian::new(i16::MIN + 1, i16::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 2), &cam), Cartesian::new(i16::MIN + 2, i16::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 1), &cam), Cartesian::new(i16::MIN + 1, i16::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 2), &cam), Cartesian::new(i16::MIN + 2, i16::MIN));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::new((10, 10), (13, 13));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 10), &cam), Cartesian::new(i16::MIN, i16::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 11), &cam), Cartesian::new(i16::MIN + 1, i16::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 12), &cam), Cartesian::new(i16::MIN + 2, i16::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(10, 13), &cam), Cartesian::new(i16::MIN + 3, i16::MIN + 3));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 10), &cam), Cartesian::new(i16::MIN, i16::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 11), &cam), Cartesian::new(i16::MIN + 1, i16::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 12), &cam), Cartesian::new(i16::MIN + 2, i16::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(11, 13), &cam), Cartesian::new(i16::MIN + 3, i16::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 10), &cam), Cartesian::new(i16::MIN, i16::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 11), &cam), Cartesian::new(i16::MIN + 1, i16::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 12), &cam), Cartesian::new(i16::MIN + 2, i16::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(12, 13), &cam), Cartesian::new(i16::MIN + 3, i16::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 10), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 11), &cam), Cartesian::new(i16::MIN + 1, i16::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 12), &cam), Cartesian::new(i16::MIN + 2, i16::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(13, 13), &cam), Cartesian::new(i16::MIN + 3, i16::MIN));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::min(), &cam), Cartesian::new(i16::MIN, i16::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u16::MAX, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(0, u16::MAX), &cam), Cartesian::max());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::max(), &cam), Cartesian::new(i16::MAX, i16::MIN));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(1, 1), &cam), Cartesian::new(i16::MIN + 1, i16::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(2, 2), &cam), Cartesian::new(i16::MIN + 2, i16::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(3, 3), &cam), Cartesian::new(i16::MIN + 3, i16::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u16::MAX - 1, u16::MAX - 1), &cam), Cartesian::new(i16::MAX - 1, i16::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u16::MAX - 2, u16::MAX - 2), &cam), Cartesian::new(i16::MAX - 2, i16::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::new(u16::MAX - 3, u16::MAX - 3), &cam), Cartesian::new(i16::MAX - 3, i16::MIN + 3));
    }
}
