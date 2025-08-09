type CartesianPoint = crate::cartesian::point::point_i32::PointI32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;
type Cam = crate::matrix::rect::rect_u32::RectU32;

pub fn matrix_in_cam_to_cartesian(point: &MatrixPoint, cam: &Cam) -> CartesianPoint {
    let x = i64::from(i32::MIN) + i64::from(point.col) - i64::from(cam.min.col);
    let y = i64::from(i32::MIN) + i64::from(cam.max.row) - i64::from(point.row);
    CartesianPoint { x: x as i32, y: y as i32 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, matrix_in_cam_to_cartesian};

    #[test]
    fn matrix_in_cam_to_cartesian_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i32::MIN, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(i32::MIN + 1, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(i32::MIN + 2, i32::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(i32::MIN, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i32::MIN + 1, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(i32::MIN + 2, i32::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::min());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(i32::MIN + 1, i32::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i32::MIN + 2, i32::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 10), &cam), CartesianPoint::of(i32::MIN, i32::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 11), &cam), CartesianPoint::of(i32::MIN + 1, i32::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 12), &cam), CartesianPoint::of(i32::MIN + 2, i32::MIN + 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 13), &cam), CartesianPoint::of(i32::MIN + 3, i32::MIN + 3));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 10), &cam), CartesianPoint::of(i32::MIN, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 11), &cam), CartesianPoint::of(i32::MIN + 1, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 12), &cam), CartesianPoint::of(i32::MIN + 2, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 13), &cam), CartesianPoint::of(i32::MIN + 3, i32::MIN + 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 10), &cam), CartesianPoint::of(i32::MIN, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 11), &cam), CartesianPoint::of(i32::MIN + 1, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 12), &cam), CartesianPoint::of(i32::MIN + 2, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 13), &cam), CartesianPoint::of(i32::MIN + 3, i32::MIN + 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 10), &cam), CartesianPoint::min());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 11), &cam), CartesianPoint::of(i32::MIN + 1, i32::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 12), &cam), CartesianPoint::of(i32::MIN + 2, i32::MIN));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 13), &cam), CartesianPoint::of(i32::MIN + 3, i32::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_edges() {
        let cam = Cam::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i32::MIN, i32::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u32::MAX, 0), &cam), CartesianPoint::min());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, u32::MAX), &cam), CartesianPoint::max());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::max(), &cam), CartesianPoint::of(i32::MAX, i32::MIN));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_sequence_min() {
        let cam = Cam::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i32::MIN + 1, i32::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i32::MIN + 2, i32::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(i32::MIN + 3, i32::MAX - 3));
    }

    #[test]
    fn matrix_in_cam_to_cartesian_sequence_max() {
        let cam = Cam::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u32::MAX - 1, u32::MAX - 1), &cam), CartesianPoint::of(i32::MAX - 1, i32::MIN + 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u32::MAX - 2, u32::MAX - 2), &cam), CartesianPoint::of(i32::MAX - 2, i32::MIN + 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u32::MAX - 3, u32::MAX - 3), &cam), CartesianPoint::of(i32::MAX - 3, i32::MIN + 3));
    }
}
