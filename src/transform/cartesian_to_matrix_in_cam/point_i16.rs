use crate::cartesian::rect::rect_i16::RectI16;

type CartesianPoint = crate::cartesian::point::point_i16::PointI16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &RectI16) -> MatrixPoint {
    let row = i32::from(cam.max.y) - i32::from(point.y);
    let col = i32::from(point.x) - i32::from(cam.min.x);
    MatrixPoint { row: row as u16, col: col as u16 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i16::RectI16;

    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = RectI16::of(-1, -1, 1, 1);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_4x4() {
        let cam = RectI16::of(-2, -2, 1, 1);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-2, -2), &cam), MatrixPoint::of(3, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-2, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-2, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-2, 1), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-1, -2), &cam), MatrixPoint::of(3, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, -2), &cam), MatrixPoint::of(3, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 2));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, -2), &cam), MatrixPoint::of(3, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 3));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_bounds() {
        let cam = RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MIN), &cam), MatrixPoint::of(u16::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX, i16::MIN), &cam), MatrixPoint::of(u16::MAX, u16::MAX));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN, i16::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX, i16::MAX), &cam), MatrixPoint::of(0, u16::MAX));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_min() {
        let cam = RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 1, i16::MIN + 1), &cam), MatrixPoint::of(u16::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 2, i16::MIN + 2), &cam), MatrixPoint::of(u16::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MIN + 3, i16::MIN + 3), &cam), MatrixPoint::of(u16::MAX - 3, 3));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_sequence_max() {
        let cam = RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX - 1, i16::MAX - 1), &cam), MatrixPoint::of(1, u16::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX - 2, i16::MAX - 2), &cam), MatrixPoint::of(2, u16::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i16::MAX - 3, i16::MAX - 3), &cam), MatrixPoint::of(3, u16::MAX - 3));
    }
}
