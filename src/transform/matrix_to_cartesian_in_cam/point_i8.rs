use crate::cartesian::rect::rect_i8::RectI8;

type CartesianPoint = crate::cartesian::point::point_i8::PointI8;
type MatrixPoint = crate::matrix::point::point_u8::PointU8;

pub fn matrix_to_cartesian_in_cam(point: &MatrixPoint, cam: &RectI8) -> CartesianPoint {
    CartesianPoint { x: cam.min.x + point.col as i8, y: cam.max.y - point.row as i8 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i8::RectI8;

    use super::{matrix_to_cartesian_in_cam, MatrixPoint, CartesianPoint};

    #[test]
    fn matrix_to_cartesian_in_cam_3x3() {
        let cam = RectI8::of(-1, -1, 1, 1);
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
        let cam = RectI8::of(-2, -2, 1, 1);
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
    fn test_matrix_to_cartesian_in_cam_cam_negative() {
        let cam = RectI8::of(-10, -5, -8, -3);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-10, -3));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(-9, -3));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(-8, -3));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-10, -4));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(-9, -4));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(-8, -4));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(-10, -5));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(-9, -5));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(-8, -5));
    }

    #[test]
    fn matrix_to_cartesian_in_cam_cam_positive() {
        let cam = RectI8::of(3, 5, 5, 7);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(3, 7));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(4, 7));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(5, 7));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(3, 6));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(4, 6));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(5, 6));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(3, 5));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(4, 5));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(5, 5));
    }

}
