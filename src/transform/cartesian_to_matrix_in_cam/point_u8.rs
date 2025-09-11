type CartesianPoint = crate::cartesian::point::point_u8::PointU8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;
type Cam = crate::matrix::rect::rect_u8::Rect;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    MatrixPoint { row: u8::MAX - point.y + cam.min.row, col: point.x + cam.min.col }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX - 2), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX - 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, u8::MAX - 2), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, u8::MAX - 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, u8::MAX), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, u8::MAX - 2), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, u8::MAX - 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, u8::MAX), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX - 3), &cam), MatrixPoint::of(13, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX - 2), &cam), MatrixPoint::of(12, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX - 1), &cam), MatrixPoint::of(11, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX), &cam), MatrixPoint::of(10, 10));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, u8::MAX - 3), &cam), MatrixPoint::of(13, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, u8::MAX - 2), &cam), MatrixPoint::of(12, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, u8::MAX - 1), &cam), MatrixPoint::of(11, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, u8::MAX), &cam), MatrixPoint::of(10, 11));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, u8::MAX - 3), &cam), MatrixPoint::of(13, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, u8::MAX - 2), &cam), MatrixPoint::of(12, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, u8::MAX - 1), &cam), MatrixPoint::of(11, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, u8::MAX), &cam), MatrixPoint::of(10, 12));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, u8::MAX - 3), &cam), MatrixPoint::of(13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, u8::MAX - 2), &cam), MatrixPoint::of(12, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, u8::MAX - 1), &cam), MatrixPoint::of(11, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, u8::MAX), &cam), MatrixPoint::of(10, 13));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(u8::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX, 0), &cam), MatrixPoint::max());
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::max(), &cam), MatrixPoint::of(0, u8::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(u8::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(u8::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 3), &cam), MatrixPoint::of(u8::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX - 1, u8::MAX - 1), &cam), MatrixPoint::of(1, u8::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX - 2, u8::MAX - 2), &cam), MatrixPoint::of(2, u8::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX - 3, u8::MAX - 3), &cam), MatrixPoint::of(3, u8::MAX - 3));
    }
}
