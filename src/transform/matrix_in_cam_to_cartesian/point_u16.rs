type CartesianPoint = crate::cartesian::point::point_u16::PointU16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;
type Cam = crate::matrix::rect::rect_u16::RectU16;

pub fn matrix_in_cam_to_cartesian(point: &MatrixPoint, cam: &Cam) -> CartesianPoint {
    CartesianPoint { x: point.col - cam.min.col, y: cam.max.row - point.row }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, matrix_in_cam_to_cartesian};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(0, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(1, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(2, 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(1, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(2, 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(1, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(2, 0));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 10), &cam), CartesianPoint::of(0, 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 11), &cam), CartesianPoint::of(1, 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 12), &cam), CartesianPoint::of(2, 3));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(10, 13), &cam), CartesianPoint::of(3, 3));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 10), &cam), CartesianPoint::of(0, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 11), &cam), CartesianPoint::of(1, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 12), &cam), CartesianPoint::of(2, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(11, 13), &cam), CartesianPoint::of(3, 2));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 10), &cam), CartesianPoint::of(0, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 11), &cam), CartesianPoint::of(1, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 12), &cam), CartesianPoint::of(2, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(12, 13), &cam), CartesianPoint::of(3, 1));

        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 10), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 11), &cam), CartesianPoint::of(1, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 12), &cam), CartesianPoint::of(2, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(13, 13), &cam), CartesianPoint::of(3, 0));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(0, u16::MAX));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u16::MAX, 0), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(0, u16::MAX), &cam), CartesianPoint::max());
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::max(), &cam), CartesianPoint::of(u16::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(1, u16::MAX - 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(2, u16::MAX - 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(3, u16::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u16::MAX - 1, u16::MAX - 1), &cam), CartesianPoint::of(u16::MAX - 1, 1));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u16::MAX - 2, u16::MAX - 2), &cam), CartesianPoint::of(u16::MAX - 2, 2));
        assert_eq!(matrix_in_cam_to_cartesian(&MatrixPoint::of(u16::MAX - 3, u16::MAX - 3), &cam), CartesianPoint::of(u16::MAX - 3, 3));
    }
}
