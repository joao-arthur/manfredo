type CartesianPoint = crate::cartesian::point::point_u32::PointU32;
type MatrixPoint = crate::matrix::point::point_u32::PointU32;
type Cam = crate::cartesian::rect::rect_u32::RectU32;

pub fn matrix_to_cartesian_in_cam(point: &MatrixPoint, cam: &Cam) -> CartesianPoint {
    CartesianPoint { x: point.col + cam.min.x, y: cam.max.y - point.row }
}

#[cfg(test)]
mod tests {
    use super::{Cam, CartesianPoint, MatrixPoint, matrix_to_cartesian_in_cam};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
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
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(10, 13));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(11, 13));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(12, 13));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 3), &cam), CartesianPoint::of(13, 13));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(10, 12));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(11, 12));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(12, 12));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 3), &cam), CartesianPoint::of(13, 12));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(10, 11));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(11, 11));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(12, 11));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 3), &cam), CartesianPoint::of(13, 11));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 0), &cam), CartesianPoint::of(10, 10));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 1), &cam), CartesianPoint::of(11, 10));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 2), &cam), CartesianPoint::of(12, 10));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(13, 10));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(0, u32::MAX));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX, 0), &cam), CartesianPoint::of(0, 0));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, u32::MAX), &cam), CartesianPoint::max());
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::max(), &cam), CartesianPoint::of(u32::MAX, 0));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(1, u32::MAX - 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(2, u32::MAX - 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(3, u32::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 1, u32::MAX - 1), &cam), CartesianPoint::of(u32::MAX - 1, 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 2, u32::MAX - 2), &cam), CartesianPoint::of(u32::MAX - 2, 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u32::MAX - 3, u32::MAX - 3), &cam), CartesianPoint::of(u32::MAX - 3, 3));
    }
}
