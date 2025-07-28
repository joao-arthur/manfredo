type CartesianPoint = crate::cartesian::point::point_i8::PointI8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;
type Cam = crate::matrix::rect::rect_u8::RectU8;

pub fn matrix_in_cam_to_cartesian(point: &MatrixPoint, cam: &Cam) -> CartesianPoint {
    let x = i16::from(i8::MIN) + i16::from(point.col) - i16::from(cam.min.col);
    let y = i16::from(i8::MIN) + i16::from(cam.max.row) - i16::from(point.row);
    CartesianPoint { x: x as i8, y: y as i8 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, matrix_in_cam_to_cartesian};

    #[test]
    fn matrix_in_cam_to_cartesian_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i8::MIN, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(i8::MIN + 1, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(i8::MIN + 2, i8::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(i8::MIN, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i8::MIN + 1, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(i8::MIN + 2, i8::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(i8::MIN, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(i8::MIN + 1, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i8::MIN + 2, i8::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 10), &cam), CartesianPoint::of(i8::MIN, i8::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 11), &cam), CartesianPoint::of(i8::MIN + 1, i8::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 12), &cam), CartesianPoint::of(i8::MIN + 2, i8::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 13), &cam), CartesianPoint::of(i8::MIN + 3, i8::MIN + 3));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 10), &cam), CartesianPoint::of(i8::MIN, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 11), &cam), CartesianPoint::of(i8::MIN + 1, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 12), &cam), CartesianPoint::of(i8::MIN + 2, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 13), &cam), CartesianPoint::of(i8::MIN + 3, i8::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 10), &cam), CartesianPoint::of(i8::MIN, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 11), &cam), CartesianPoint::of(i8::MIN + 1, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 12), &cam), CartesianPoint::of(i8::MIN + 2, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 13), &cam), CartesianPoint::of(i8::MIN + 3, i8::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 10), &cam), CartesianPoint::of(i8::MIN, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 11), &cam), CartesianPoint::of(i8::MIN + 1, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 12), &cam), CartesianPoint::of(i8::MIN + 2, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 13), &cam), CartesianPoint::of(i8::MIN + 3, i8::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_edges() {
        let cam = Cam::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i8::MIN, i8::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u8::MAX, 0), &cam), CartesianPoint::of(i8::MIN, i8::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, u8::MAX), &cam), CartesianPoint::of(i8::MAX, i8::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u8::MAX, u8::MAX), &cam), CartesianPoint::of(i8::MAX, i8::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_sequence_min() {
        let cam = Cam::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i8::MIN + 1, i8::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i8::MIN + 2, i8::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(i8::MIN + 3, i8::MAX - 3));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_sequence_max() {
        let cam = Cam::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u8::MAX - 1, u8::MAX - 1), &cam), CartesianPoint::of(i8::MAX - 1, i8::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u8::MAX - 2, u8::MAX - 2), &cam), CartesianPoint::of(i8::MAX - 2, i8::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u8::MAX - 3, u8::MAX - 3), &cam), CartesianPoint::of(i8::MAX - 3, i8::MIN + 3));
    }
}
