use crate::cartesian::rect::rect_i32::RectI32;

type CartesianPoint = crate::cartesian::point::point_i32::PointI32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;

pub fn matrix_to_cartesian_in_cam(point: &MatrixPoint, cam: &RectI32) -> CartesianPoint {
    let x = i64::from(cam.min.x) + i64::from(point.col);
    let y = i64::from(cam.max.y) - i64::from(point.row);
    CartesianPoint { x: x as i32, y: y as i32 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i32::RectI32;

    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian_in_cam};

    #[test]
    fn matrix_to_cartesian_in_cam_3x3() {
        let cam = RectI32::of(-1, -1, 1, 1);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(1, 1));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-1, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(1, 0));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(-1, -1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(0, -1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(1, -1));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_4x4() {
        let cam = RectI32::of(-2, -2, 1, 1);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-2, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(-1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 3), &cam), CartesianPoint::of(1, 1));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-2, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(-1, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 3), &cam), CartesianPoint::of(1, 0));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(-2, -1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(-1, -1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(0, -1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 3), &cam), CartesianPoint::of(1, -1));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 0), &cam), CartesianPoint::of(-2, -2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 1), &cam), CartesianPoint::of(-1, -2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 2), &cam), CartesianPoint::of(0, -2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(1, -2));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_edges() {
        let cam = RectI32::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i32::MIN, i32::MAX));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX, 0), &cam), CartesianPoint::of(i32::MIN, i32::MIN));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, u32::MAX), &cam), CartesianPoint::of(i32::MAX, i32::MAX));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX, u32::MAX), &cam), CartesianPoint::of(i32::MAX, i32::MIN));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_sequence_min() {
        let cam = RectI32::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i32::MIN + 1, i32::MAX - 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i32::MIN + 2, i32::MAX - 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(i32::MIN + 3, i32::MAX - 3));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_sequence_max() {
        let cam = RectI32::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 1, u32::MAX - 1), &cam), CartesianPoint::of(i32::MAX - 1, i32::MIN + 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 2, u32::MAX - 2), &cam), CartesianPoint::of(i32::MAX - 2, i32::MIN + 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 3, u32::MAX - 3), &cam), CartesianPoint::of(i32::MAX - 3, i32::MIN + 3));
    }
}
