type CartesianPoint = crate::cartesian::point::point_i16::PointI16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;
type Cam = crate::matrix::rect::rect_u16::RectU16;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    let row = i32::from(i16::MAX) - i32::from(point.y) + i32::from(cam.min.row);
    let col = i32::from(i16::MAX) + i32::from(point.x) + 1 + i32::from(cam.min.col);
    MatrixPoint { row: row as u16, col: col as u16 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX - 2), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX - 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MAX - 2), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MAX - 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MAX), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MAX - 2), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MAX - 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MAX), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX - 3), &cam), MatrixPoint::of(13, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX - 2), &cam), MatrixPoint::of(12, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX - 1), &cam), MatrixPoint::of(11, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX), &cam), MatrixPoint::of(10, 10));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MAX - 3), &cam), MatrixPoint::of(13, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MAX - 2), &cam), MatrixPoint::of(12, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MAX - 1), &cam), MatrixPoint::of(11, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MAX), &cam), MatrixPoint::of(10, 11));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MAX - 3), &cam), MatrixPoint::of(13, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MAX - 2), &cam), MatrixPoint::of(12, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MAX - 1), &cam), MatrixPoint::of(11, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MAX), &cam), MatrixPoint::of(10, 12));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 3, i16::MAX - 3), &cam), MatrixPoint::of(13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 3, i16::MAX - 2), &cam), MatrixPoint::of(12, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 3, i16::MAX - 1), &cam), MatrixPoint::of(11, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 3, i16::MAX), &cam), MatrixPoint::of(10, 13));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_bounds() {
        let cam = Cam::of(0, 0, u16::MAX, u16::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MIN), &cam), MatrixPoint::of(u16::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX, i16::MIN), &cam), MatrixPoint::of(u16::MAX, u16::MAX));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX, i16::MAX), &cam), MatrixPoint::of(0, u16::MAX));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_min() {
        let cam = Cam::of(0, 0, u16::MAX, u16::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MIN + 1), &cam), MatrixPoint::of(u16::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MIN + 2), &cam), MatrixPoint::of(u16::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 3, i16::MIN + 3), &cam), MatrixPoint::of(u16::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_max() {
        let cam = Cam::of(0, 0, u16::MAX, u16::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX - 1, i16::MAX - 1), &cam), MatrixPoint::of(1, u16::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX - 2, i16::MAX - 2), &cam), MatrixPoint::of(2, u16::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX - 3, i16::MAX - 3), &cam), MatrixPoint::of(3, u16::MAX - 3));
    }
}
