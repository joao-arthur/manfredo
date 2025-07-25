use crate::cartesian::rect::rect_i8::RectI8;

type CartesianPoint = crate::cartesian::point::point_i8::PointI8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &RectI8) -> MatrixPoint {
    let row = i16::from(cam.max.y) - i16::from(point.y);
    let col = i16::from(point.x) - i16::from(cam.min.x);
    MatrixPoint { row: row as u8, col: col as u8 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i8::RectI8;

    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_3x3() {
        let cam = RectI8::of(-1, -1, 1, 1);
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
        let cam = RectI8::of(-2, -2, 1, 1);
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
    fn cartesian_to_matrix_in_cam_min_cam() {
        let cam = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN, i8::MIN), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN, i8::MIN + 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN, i8::MIN + 2), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN + 1, i8::MIN), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN + 1, i8::MIN + 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN + 1, i8::MIN + 2), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN + 2, i8::MIN), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN + 2, i8::MIN + 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MIN + 2, i8::MIN + 2), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_max_cam() {
        let cam = RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX - 2, i8::MAX - 2), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX - 2, i8::MAX - 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX - 2, i8::MAX), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX - 1, i8::MAX - 2), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX - 1, i8::MAX - 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX - 1, i8::MAX), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX, i8::MAX - 2), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX, i8::MAX - 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(i8::MAX, i8::MAX), &cam), MatrixPoint::of(0, 2));
    }
}
