type CartesianPoint = crate::cartesian::point::point_i16::PointI16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;
type Cam = crate::cartesian::rect::rect_i16::RectI16;

pub fn matrix_to_cartesian_in_cam(point: &MatrixPoint, cam: &Cam) -> CartesianPoint {
    let x = i32::from(point.col) + i32::from(cam.min.x);
    let y = i32::from(cam.max.y) - i32::from(point.row);
    CartesianPoint { x: x as i16, y: y as i16 }
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
        let cam = Cam::of(-13, -13, -10, -10);
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(-13, -10));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 1), &cam), CartesianPoint::of(-12, -10));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 2), &cam), CartesianPoint::of(-11, -10));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 3), &cam), CartesianPoint::of(-10, -10));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 0), &cam), CartesianPoint::of(-13, -11));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(-12, -11));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 2), &cam), CartesianPoint::of(-11, -11));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 3), &cam), CartesianPoint::of(-10, -11));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 0), &cam), CartesianPoint::of(-13, -12));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 1), &cam), CartesianPoint::of(-12, -12));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(-11, -12));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 3), &cam), CartesianPoint::of(-10, -12));

        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 0), &cam), CartesianPoint::of(-13, -13));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 1), &cam), CartesianPoint::of(-12, -13));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 2), &cam), CartesianPoint::of(-11, -13));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(-10, -13));
    }

    #[test]
    fn edges() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, 0), &cam), CartesianPoint::of(i16::MIN, i16::MAX));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u16::MAX, 0), &cam), CartesianPoint::min());
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(0, u16::MAX), &cam), CartesianPoint::max());
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::max(), &cam), CartesianPoint::of(i16::MAX, i16::MIN));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(1, 1), &cam), CartesianPoint::of(i16::MIN + 1, i16::MAX - 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(2, 2), &cam), CartesianPoint::of(i16::MIN + 2, i16::MAX - 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(3, 3), &cam), CartesianPoint::of(i16::MIN + 3, i16::MAX - 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u16::MAX - 1, u16::MAX - 1), &cam), CartesianPoint::of(i16::MAX - 1, i16::MIN + 1));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u16::MAX - 2, u16::MAX - 2), &cam), CartesianPoint::of(i16::MAX - 2, i16::MIN + 2));
        assert_eq!(matrix_to_cartesian_in_cam(&MatrixPoint::of(u16::MAX - 3, u16::MAX - 3), &cam), CartesianPoint::of(i16::MAX - 3, i16::MIN + 3));
    }
}
