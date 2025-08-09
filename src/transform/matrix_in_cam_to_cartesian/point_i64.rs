type CartesianPoint = crate::cartesian::point::point_i64::PointI64;
type MatrixPoint = crate::matrix::point::point_u64::PointU64;
type Cam = crate::matrix::rect::rect_u64::RectU64;

pub fn matrix_in_cam_to_cartesian(point: &MatrixPoint, cam: &Cam) -> CartesianPoint {
    let x = i128::from(i64::MIN) + i128::from(point.col) - i128::from(cam.min.col);
    let y = i128::from(i64::MIN) + i128::from(cam.max.row) - i128::from(point.row);
    CartesianPoint { x: x as i64, y: y as i64 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, matrix_in_cam_to_cartesian};

    #[test]
    fn matrix_in_cam_to_cartesian_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i64::MIN, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(i64::MIN + 1, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(i64::MIN + 2, i64::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(i64::MIN, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i64::MIN + 1, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(i64::MIN + 2, i64::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::min());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(i64::MIN + 1, i64::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i64::MIN + 2, i64::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 10), &cam), CartesianPoint::of(i64::MIN, i64::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 11), &cam), CartesianPoint::of(i64::MIN + 1, i64::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 12), &cam), CartesianPoint::of(i64::MIN + 2, i64::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 13), &cam), CartesianPoint::of(i64::MIN + 3, i64::MIN + 3));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 10), &cam), CartesianPoint::of(i64::MIN, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 11), &cam), CartesianPoint::of(i64::MIN + 1, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 12), &cam), CartesianPoint::of(i64::MIN + 2, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 13), &cam), CartesianPoint::of(i64::MIN + 3, i64::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 10), &cam), CartesianPoint::of(i64::MIN, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 11), &cam), CartesianPoint::of(i64::MIN + 1, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 12), &cam), CartesianPoint::of(i64::MIN + 2, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 13), &cam), CartesianPoint::of(i64::MIN + 3, i64::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 10), &cam), CartesianPoint::min());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 11), &cam), CartesianPoint::of(i64::MIN + 1, i64::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 12), &cam), CartesianPoint::of(i64::MIN + 2, i64::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 13), &cam), CartesianPoint::of(i64::MIN + 3, i64::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_edges() {
        let cam = Cam::of(0, 0, u64::MAX, u64::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i64::MIN, i64::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u64::MAX, 0), &cam), CartesianPoint::min());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, u64::MAX), &cam), CartesianPoint::max());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::max(), &cam), CartesianPoint::of(i64::MAX, i64::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_sequence_min() {
        let cam = Cam::of(0, 0, u64::MAX, u64::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i64::MIN + 1, i64::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i64::MIN + 2, i64::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(i64::MIN + 3, i64::MAX - 3));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_sequence_max() {
        let cam = Cam::of(0, 0, u64::MAX, u64::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u64::MAX - 1, u64::MAX - 1), &cam), CartesianPoint::of(i64::MAX - 1, i64::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u64::MAX - 2, u64::MAX - 2), &cam), CartesianPoint::of(i64::MAX - 2, i64::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u64::MAX - 3, u64::MAX - 3), &cam), CartesianPoint::of(i64::MAX - 3, i64::MIN + 3));
    }
}
