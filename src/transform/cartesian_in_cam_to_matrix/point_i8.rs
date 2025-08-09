type CartesianPoint = crate::cartesian::point::point_i8::PointI8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;
type Cam = crate::cartesian::rect::rect_i8::RectI8;

pub fn cartesian_in_cam_to_matrix(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    let row = i16::from(cam.max.y) - i16::from(point.y);
    let col = i16::from(point.x) - i16::from(cam.min.x);
    MatrixPoint { row: row as u8, col: col as u8 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_in_cam_to_matrix};

    #[test]
    fn cartesian_in_cam_to_matrix_3x3() {
        let cam = Cam::of(-1, -1, 1, 1);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_in_cam_to_matrix_4x4() {
        let cam = Cam::of(-2, -2, 1, 1);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, -2), &cam), MatrixPoint::of(3, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, -1), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, 0), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-2, 1), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, -2), &cam), MatrixPoint::of(3, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, -1), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 0), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(-1, 1), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, -2), &cam), MatrixPoint::of(3, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, -1), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(0, 2));

        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, -2), &cam), MatrixPoint::of(3, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, -1), &cam), MatrixPoint::of(2, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(1, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(0, 3));
    }

    #[test]
    fn cartesian_in_cam_to_matrix_bounds() {
        let cam = Cam::of(i8::MIN, i8::MIN, i8::MAX, i8::MAX);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::min(), &cam), MatrixPoint::of(u8::MAX, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MAX, i8::MIN), &cam), MatrixPoint::max());
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MIN, i8::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::max(), &cam), MatrixPoint::of(0, u8::MAX));
    }

    #[test]
    fn cartesian_in_cam_to_matrix_sequence_min() {
        let cam = Cam::of(i8::MIN, i8::MIN, i8::MAX, i8::MAX);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MIN + 1, i8::MIN + 1), &cam), MatrixPoint::of(u8::MAX - 1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MIN + 2, i8::MIN + 2), &cam), MatrixPoint::of(u8::MAX - 2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MIN + 3, i8::MIN + 3), &cam), MatrixPoint::of(u8::MAX - 3, 3));
    }

    #[test]
    fn cartesian_in_cam_to_matrix_sequence_max() {
        let cam = Cam::of(i8::MIN, i8::MIN, i8::MAX, i8::MAX);
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MAX - 1, i8::MAX - 1), &cam), MatrixPoint::of(1, u8::MAX - 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MAX - 2, i8::MAX - 2), &cam), MatrixPoint::of(2, u8::MAX - 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i8::MAX - 3, i8::MAX - 3), &cam), MatrixPoint::of(3, u8::MAX - 3));
    }
}
