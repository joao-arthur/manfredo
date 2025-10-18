type Cartesian = crate::cartesian::d2::point::point_u32::Point;
type Matrix = crate::matrix::d2::point::point_u32::Point;
type Cam = crate::matrix::d2::rect::rect_u32::Rect;

pub fn matrix_in_cam_to_cartesian(point: &Matrix, cam: &Cam) -> Cartesian {
    Cartesian { x: point.col - cam.min.col, y: cam.max.row - point.row }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, matrix_in_cam_to_cartesian};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 0), &cam), Cartesian::of(0, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 1), &cam), Cartesian::of(1, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 2), &cam), Cartesian::of(2, 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 0), &cam), Cartesian::of(0, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 1), &cam), Cartesian::of(1, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 2), &cam), Cartesian::of(2, 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 1), &cam), Cartesian::of(1, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 2), &cam), Cartesian::of(2, 0));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 10), &cam), Cartesian::of(0, 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 11), &cam), Cartesian::of(1, 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 12), &cam), Cartesian::of(2, 3));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(10, 13), &cam), Cartesian::of(3, 3));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 10), &cam), Cartesian::of(0, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 11), &cam), Cartesian::of(1, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 12), &cam), Cartesian::of(2, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(11, 13), &cam), Cartesian::of(3, 2));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 10), &cam), Cartesian::of(0, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 11), &cam), Cartesian::of(1, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 12), &cam), Cartesian::of(2, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(12, 13), &cam), Cartesian::of(3, 1));

        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 10), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 11), &cam), Cartesian::of(1, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 12), &cam), Cartesian::of(2, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(13, 13), &cam), Cartesian::of(3, 0));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, 0), &cam), Cartesian::of(0, u32::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX, 0), &cam), Cartesian::min());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(0, u32::MAX), &cam), Cartesian::max());
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::max(), &cam), Cartesian::of(u32::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(1, 1), &cam), Cartesian::of(1, u32::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(2, 2), &cam), Cartesian::of(2, u32::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(3, 3), &cam), Cartesian::of(3, u32::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX - 1, u32::MAX - 1), &cam), Cartesian::of(u32::MAX - 1, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX - 2, u32::MAX - 2), &cam), Cartesian::of(u32::MAX - 2, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&Matrix::of(u32::MAX - 3, u32::MAX - 3), &cam), Cartesian::of(u32::MAX - 3, 3));
    }
}
