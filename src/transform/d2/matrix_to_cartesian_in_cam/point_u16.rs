type Cartesian = crate::cartesian::d2::point::point_u16::Point;
type Matrix = crate::matrix::d2::point::point_u16::Point;
type Cam = crate::cartesian::d2::rect::rect_u16::Rect;

pub fn matrix_to_cartesian_in_cam(point: &Matrix, cam: &Cam) -> Cartesian {
    Cartesian { x: point.col + cam.min.x, y: cam.max.y - point.row }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, matrix_to_cartesian_in_cam};

    #[test]
    fn test_3x3() {
        let cam = Cam::new((0, 0), (2, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::min(), &cam), Cartesian::new(0, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(0, 1), &cam), Cartesian::new(1, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(0, 2), &cam), Cartesian::new(2, 2));

        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 0), &cam), Cartesian::new(0, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 1), &cam), Cartesian::new(1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 2), &cam), Cartesian::new(2, 1));

        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 0), &cam), Cartesian::min());
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 1), &cam), Cartesian::new(1, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 2), &cam), Cartesian::new(2, 0));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::new((10, 10), (13, 13));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::min(), &cam), Cartesian::new(10, 13));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(0, 1), &cam), Cartesian::new(11, 13));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(0, 2), &cam), Cartesian::new(12, 13));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(0, 3), &cam), Cartesian::new(13, 13));

        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 0), &cam), Cartesian::new(10, 12));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 1), &cam), Cartesian::new(11, 12));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 2), &cam), Cartesian::new(12, 12));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 3), &cam), Cartesian::new(13, 12));

        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 0), &cam), Cartesian::new(10, 11));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 1), &cam), Cartesian::new(11, 11));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 2), &cam), Cartesian::new(12, 11));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 3), &cam), Cartesian::new(13, 11));

        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(3, 0), &cam), Cartesian::new(10, 10));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(3, 1), &cam), Cartesian::new(11, 10));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(3, 2), &cam), Cartesian::new(12, 10));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(3, 3), &cam), Cartesian::new(13, 10));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::min(), &cam), Cartesian::new(0, u16::MAX));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(u16::MAX, 0), &cam), Cartesian::min());
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(0, u16::MAX), &cam), Cartesian::max());
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::max(), &cam), Cartesian::new(u16::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(1, 1), &cam), Cartesian::new(1, u16::MAX - 1));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(2, 2), &cam), Cartesian::new(2, u16::MAX - 2));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(3, 3), &cam), Cartesian::new(3, u16::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(u16::MAX - 1, u16::MAX - 1), &cam), Cartesian::new(u16::MAX - 1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(u16::MAX - 2, u16::MAX - 2), &cam), Cartesian::new(u16::MAX - 2, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&Matrix::new(u16::MAX - 3, u16::MAX - 3), &cam), Cartesian::new(u16::MAX - 3, 3));
    }
}
