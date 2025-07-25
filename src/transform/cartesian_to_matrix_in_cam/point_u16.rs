use crate::cartesian::rect::rect_u16::RectU16;

type CartesianPoint = crate::cartesian::point::point_u16::PointU16;
type MatrixPoint = crate::matrix::point::point_u16::PointU16;

pub fn cartesian_to_matrix_in_cam(point: &CartesianPoint, cam: &RectU16) -> MatrixPoint {
    let row = i32::from(cam.max.y) - i32::from(point.y);
    let col = i32::from(point.x) - i32::from(cam.min.x);
    MatrixPoint { row: row as u16, col: col as u16 }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u16::RectU16;

    use super::{CartesianPoint, MatrixPoint, cartesian_to_matrix_in_cam};

    #[test]
    fn cartesian_to_matrix_in_cam_min() {
        let cam = RectU16::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 0), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(0, 2), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 0), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(1, 2), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 0), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(2, 2), &cam), MatrixPoint::of(0, 2));
    }

    #[test]
    fn cartesian_to_matrix_in_cam_max() {
        let cam = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX - 2, u16::MAX - 2), &cam), MatrixPoint::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX - 2, u16::MAX - 1), &cam), MatrixPoint::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX - 2, u16::MAX), &cam), MatrixPoint::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX - 1, u16::MAX - 2), &cam), MatrixPoint::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX - 1, u16::MAX - 1), &cam), MatrixPoint::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX - 1, u16::MAX), &cam), MatrixPoint::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX, u16::MAX - 2), &cam), MatrixPoint::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX, u16::MAX - 1), &cam), MatrixPoint::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&CartesianPoint::of(u16::MAX, u16::MAX), &cam), MatrixPoint::of(0, 2));
    }
}
