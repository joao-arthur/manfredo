type Cartesian = crate::cartesian::d2::point::point_i32::Point;
type Matrix = crate::matrix::d2::point::point_u32::Point;
type Cam = crate::cartesian::d2::rect::rect_i32::Rect;

pub fn cartesian_in_cam_to_matrix(point: &Cartesian, cam: &Cam) -> Matrix {
    let row = i64::from(cam.max.y) - i64::from(point.y);
    let col = i64::from(point.x) - i64::from(cam.min.x);
    Matrix { row: row as u32, col: col as u32 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, cartesian_in_cam_to_matrix};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(-1, -1, 1, 1);
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-1, -1), &cam), Matrix::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-1, 0), &cam), Matrix::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-1, 1), &cam), Matrix::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(0, -1), &cam), Matrix::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(0, 0), &cam), Matrix::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(0, 1), &cam), Matrix::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(1, -1), &cam), Matrix::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(1, 0), &cam), Matrix::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(1, 1), &cam), Matrix::of(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(-2, -2, 1, 1);
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-2, -2), &cam), Matrix::of(3, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-2, -1), &cam), Matrix::of(2, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-2, 0), &cam), Matrix::of(1, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-2, 1), &cam), Matrix::of(0, 0));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-1, -2), &cam), Matrix::of(3, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-1, -1), &cam), Matrix::of(2, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-1, 0), &cam), Matrix::of(1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(-1, 1), &cam), Matrix::of(0, 1));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(0, -2), &cam), Matrix::of(3, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(0, -1), &cam), Matrix::of(2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(0, 0), &cam), Matrix::of(1, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(0, 1), &cam), Matrix::of(0, 2));

        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(1, -2), &cam), Matrix::of(3, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(1, -1), &cam), Matrix::of(2, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(1, 0), &cam), Matrix::of(1, 3));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(1, 1), &cam), Matrix::of(0, 3));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::min(), &cam), Matrix::of(u32::MAX, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MAX, i32::MIN), &cam), Matrix::max());
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MIN, i32::MAX), &cam), Matrix::of(0, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::max(), &cam), Matrix::of(0, u32::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MIN + 1, i32::MIN + 1), &cam), Matrix::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MIN + 2, i32::MIN + 2), &cam), Matrix::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MIN + 3, i32::MIN + 3), &cam), Matrix::of(u32::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MAX - 1, i32::MAX - 1), &cam), Matrix::of(1, u32::MAX - 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MAX - 2, i32::MAX - 2), &cam), Matrix::of(2, u32::MAX - 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i32::MAX - 3, i32::MAX - 3), &cam), Matrix::of(3, u32::MAX - 3));
    }
}
