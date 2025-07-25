use crate::cartesian::rect::rect_u32::RectU32;

type CartesianPoint = crate::cartesian::point::point_u32::PointU32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;

pub fn matrix_to_cartesian_in_cam(point: &MatrixPoint, cam: &RectU32) -> CartesianPoint {
    let x = i64::from(point.col) - i64::from(cam.min.x);
    let y = i64::from(cam.max.y) - i64::from(point.row);
    CartesianPoint { x: x as u32, y: y as u32 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u32::RectU32;

    use super::{CartesianPoint, MatrixPoint, matrix_to_cartesian_in_cam};

    #[test]
    fn matrix_to_cartesian_in_cam_3x3() {
        let cam = RectU32::of(0, 0, 2, 2);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(0, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(1, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(2, 2));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(2, 1));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(1, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(2, 0));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_4x4() {
        let cam = RectU32::of(10, 10, 13, 13);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(10, 10), &cam), CartesianPoint::of(0, 3));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(10, 11), &cam), CartesianPoint::of(1, 3));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(10, 12), &cam), CartesianPoint::of(2, 3));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(10, 13), &cam), CartesianPoint::of(3, 3));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(11, 10), &cam), CartesianPoint::of(0, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(11, 11), &cam), CartesianPoint::of(1, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(11, 12), &cam), CartesianPoint::of(2, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(11, 13), &cam), CartesianPoint::of(3, 2));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(12, 10), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(12, 11), &cam), CartesianPoint::of(1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(12, 12), &cam), CartesianPoint::of(2, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(12, 13), &cam), CartesianPoint::of(3, 1));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(13, 10), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(13, 11), &cam), CartesianPoint::of(1, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(13, 12), &cam), CartesianPoint::of(2, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(13, 13), &cam), CartesianPoint::of(3, 0));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_edges() {
        let cam = RectU32::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(0, u32::MAX));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX, 0), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, u32::MAX), &cam), CartesianPoint::of(u32::MAX, u32::MAX));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX, u32::MAX), &cam), CartesianPoint::of(u32::MAX, 0));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_sequence_min() {
        let cam = RectU32::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(1, u32::MAX - 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(2, u32::MAX - 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(3, u32::MAX - 3));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_sequence_max() {
        let cam = RectU32::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 1, u32::MAX - 1), &cam), CartesianPoint::of(u32::MAX - 1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 2, u32::MAX - 2), &cam), CartesianPoint::of(u32::MAX - 2, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 3, u32::MAX - 3), &cam), CartesianPoint::of(u32::MAX - 3, 3));
    }
}
