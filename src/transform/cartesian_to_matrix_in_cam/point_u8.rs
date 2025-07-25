use crate::cartesian::rect::rect_u8::RectU8;

type CartesianPoint = crate::cartesian::point::point_u8::PointU8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &RectU8) -> MatrixPoint {
    let row = i16::from(cam.max.y) - i16::from(point.y);
    let col = i16::from(point.x) - i16::from(cam.min.x);
    MatrixPoint { row: row as u8, col: col as u8 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u8::RectU8;

    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = RectU8::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 2), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 2), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 0), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_4x4() {
        let cam = RectU8::of(0, 0, 3, 3);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(3, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 2), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 3), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(3, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 2), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 3), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 0), &cam), MatrixPoint::of(3, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 3), &cam), MatrixPoint::of(0, 2));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 0), &cam), MatrixPoint::of(3, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 1), &cam), MatrixPoint::of(2, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 2), &cam), MatrixPoint::of(1, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 3), &cam), MatrixPoint::of(0, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_bounds() {
        let cam = RectU8::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(u8::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX, 0), &cam), MatrixPoint::of(u8::MAX, u8::MAX));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, u8::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX, u8::MAX), &cam), MatrixPoint::of(0, u8::MAX));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_min() {
        let cam = RectU8::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(u8::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(u8::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(3, 3), &cam), MatrixPoint::of(u8::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_max() {
        let cam = RectU8::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX - 1, u8::MAX - 1), &cam), MatrixPoint::of(1, u8::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX - 2, u8::MAX - 2), &cam), MatrixPoint::of(2, u8::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u8::MAX - 3, u8::MAX - 3), &cam), MatrixPoint::of(3, u8::MAX - 3));
    }
}
