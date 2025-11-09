type Cartesian = crate::cartesian::d2::point::point_u32::Point;
type Matrix = crate::matrix::d2::point::point_u32::Point;
type Cam = crate::cartesian::d2::rect::rect_u32::Rect;

pub fn cartesian_in_cam_to_matrix(point: &Cartesian, cam: &Cam) -> Matrix {
    Matrix { row: cam.max.y - point.y, col: point.x - cam.min.x }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, cartesian_in_cam_to_matrix};

    #[test]
    fn test_3x3() {
        let cam = Cam::new((0, 0), (2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::min(), &cam), Matrix::new(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(0, 1), &cam), Matrix::new(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(0, 2), &cam), Matrix::min());

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(1, 0), &cam), Matrix::new(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(1, 1), &cam), Matrix::new(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(1, 2), &cam), Matrix::new(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(2, 0), &cam), Matrix::new(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(2, 1), &cam), Matrix::new(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(2, 2), &cam), Matrix::new(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::new((10, 10), (13, 13));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(10, 10), &cam), Matrix::new(3, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(10, 11), &cam), Matrix::new(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(10, 12), &cam), Matrix::new(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(10, 13), &cam), Matrix::min());

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(11, 10), &cam), Matrix::new(3, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(11, 11), &cam), Matrix::new(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(11, 12), &cam), Matrix::new(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(11, 13), &cam), Matrix::new(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(12, 10), &cam), Matrix::new(3, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(12, 11), &cam), Matrix::new(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(12, 12), &cam), Matrix::new(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(12, 13), &cam), Matrix::new(0, 2));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(13, 10), &cam), Matrix::new(3, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(13, 11), &cam), Matrix::new(2, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(13, 12), &cam), Matrix::new(1, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(13, 13), &cam), Matrix::new(0, 3));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::min(), &cam), Matrix::new(u32::MAX, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(u32::MAX, 0), &cam), Matrix::max());
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(0, u32::MAX), &cam), Matrix::min());
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::max(), &cam), Matrix::new(0, u32::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(1, 1), &cam), Matrix::new(u32::MAX - 1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(2, 2), &cam), Matrix::new(u32::MAX - 2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(3, 3), &cam), Matrix::new(u32::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(u32::MAX - 1, u32::MAX - 1), &cam), Matrix::new(1, u32::MAX - 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(u32::MAX - 2, u32::MAX - 2), &cam), Matrix::new(2, u32::MAX - 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::new(u32::MAX - 3, u32::MAX - 3), &cam), Matrix::new(3, u32::MAX - 3));
    }
}
