type Cartesian = crate::cartesian::d2::point::point_u16::Point;
type Matrix = crate::matrix::d2::point::point_u16::Point;
type Cam = crate::matrix::d2::rect::rect_u16::Rect;

pub fn cartesian_to_matrix_in_cam(point: &Cartesian, cam: &Cam) -> Matrix {
    Matrix { row: u16::MAX - point.y + cam.min.row, col: point.x + cam.min.col }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, cartesian_to_matrix_in_cam};

    #[test]
    fn test_3x3() {
        let cam = Cam::new((0, 0), (2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX - 2), &cam), Matrix::new(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX - 1), &cam), Matrix::new(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX), &cam), Matrix::min());

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, u16::MAX - 2), &cam), Matrix::new(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, u16::MAX - 1), &cam), Matrix::new(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, u16::MAX), &cam), Matrix::new(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, u16::MAX - 2), &cam), Matrix::new(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, u16::MAX - 1), &cam), Matrix::new(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, u16::MAX), &cam), Matrix::new(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::new((10, 10), (13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX - 3), &cam), Matrix::new(13, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX - 2), &cam), Matrix::new(12, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX - 1), &cam), Matrix::new(11, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX), &cam), Matrix::new(10, 10));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, u16::MAX - 3), &cam), Matrix::new(13, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, u16::MAX - 2), &cam), Matrix::new(12, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, u16::MAX - 1), &cam), Matrix::new(11, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, u16::MAX), &cam), Matrix::new(10, 11));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, u16::MAX - 3), &cam), Matrix::new(13, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, u16::MAX - 2), &cam), Matrix::new(12, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, u16::MAX - 1), &cam), Matrix::new(11, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, u16::MAX), &cam), Matrix::new(10, 12));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(3, u16::MAX - 3), &cam), Matrix::new(13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(3, u16::MAX - 2), &cam), Matrix::new(12, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(3, u16::MAX - 1), &cam), Matrix::new(11, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(3, u16::MAX), &cam), Matrix::new(10, 13));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::min(), &cam), Matrix::new(u16::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(u16::MAX, 0), &cam), Matrix::max());
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(0, u16::MAX), &cam), Matrix::min());
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::max(), &cam), Matrix::new(0, u16::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(1, 1), &cam), Matrix::new(u16::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(2, 2), &cam), Matrix::new(u16::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(3, 3), &cam), Matrix::new(u16::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(u16::MAX - 1, u16::MAX - 1), &cam), Matrix::new(1, u16::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(u16::MAX - 2, u16::MAX - 2), &cam), Matrix::new(2, u16::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::new(u16::MAX - 3, u16::MAX - 3), &cam), Matrix::new(3, u16::MAX - 3));
    }
}
