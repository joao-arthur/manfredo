type CartesianPoint = crate::cartesian::point::point_i16::PointI16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;
type Cam = crate::cartesian::rect::rect_i16::RectI16;

pub fn cartesian_in_cam_to_matrix(point: &CartesianPoint, cam: &Cam) -> MatrixPoint {
    let row = i32::from(cam.max.y) - i32::from(point.y);
    let col = i32::from(point.x) - i32::from(cam.min.x);
    MatrixPoint { row: row as u16, col: col as u16 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, cartesian_in_cam_to_matrix};

    #[test]
    fn test_3x3() {
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
    fn test_4x4() {
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
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::min(), &cam), MatrixPoint::of(u16::MAX, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MAX, i16::MIN), &cam), MatrixPoint::max());
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MIN, i16::MAX), &cam), MatrixPoint::of(0, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::max(), &cam), MatrixPoint::of(0, u16::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MIN + 1, i16::MIN + 1), &cam), MatrixPoint::of(u16::MAX - 1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MIN + 2, i16::MIN + 2), &cam), MatrixPoint::of(u16::MAX - 2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MIN + 3, i16::MIN + 3), &cam), MatrixPoint::of(u16::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MAX - 1, i16::MAX - 1), &cam), MatrixPoint::of(1, u16::MAX - 1));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MAX - 2, i16::MAX - 2), &cam), MatrixPoint::of(2, u16::MAX - 2));
        assert_eq!(cartesian_in_cam_to_matrix(&CartesianPoint::of(i16::MAX - 3, i16::MAX - 3), &cam), MatrixPoint::of(3, u16::MAX - 3));
    }
}
