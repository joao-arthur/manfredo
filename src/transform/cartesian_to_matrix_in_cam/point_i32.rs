type Cartesian = crate::cartesian::point::point_i32::Point;
type Matrix = crate::matrix::point::point_u32::Point;
type Cam = crate::matrix::rect::rect_u32::Rect;

pub fn cartesian_to_matrix_in_cam(point: &Cartesian, cam: &Cam) -> Matrix {
    let row = i64::from(i32::MAX) - i64::from(point.y) + i64::from(cam.min.row);
    let col = i64::from(point.x) - i64::from(i32::MIN) + i64::from(cam.min.col);
    Matrix { row: row as u32, col: col as u32 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, cartesian_to_matrix_in_cam};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX - 2), &cam), Matrix::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX - 1), &cam), Matrix::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX), &cam), Matrix::of(0, 0));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MAX - 2), &cam), Matrix::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MAX - 1), &cam), Matrix::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MAX), &cam), Matrix::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MAX - 2), &cam), Matrix::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MAX - 1), &cam), Matrix::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MAX), &cam), Matrix::of(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX - 3), &cam), Matrix::of(13, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX - 2), &cam), Matrix::of(12, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX - 1), &cam), Matrix::of(11, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX), &cam), Matrix::of(10, 10));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MAX - 3), &cam), Matrix::of(13, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MAX - 2), &cam), Matrix::of(12, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MAX - 1), &cam), Matrix::of(11, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MAX), &cam), Matrix::of(10, 11));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MAX - 3), &cam), Matrix::of(13, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MAX - 2), &cam), Matrix::of(12, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MAX - 1), &cam), Matrix::of(11, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MAX), &cam), Matrix::of(10, 12));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 3, i32::MAX - 3), &cam), Matrix::of(13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 3, i32::MAX - 2), &cam), Matrix::of(12, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 3, i32::MAX - 1), &cam), Matrix::of(11, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 3, i32::MAX), &cam), Matrix::of(10, 13));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::min(), &cam), Matrix::of(u32::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MAX, i32::MIN), &cam), Matrix::max());
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN, i32::MAX), &cam), Matrix::of(0, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::max(), &cam), Matrix::of(0, u32::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 1, i32::MIN + 1), &cam), Matrix::of(u32::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 2, i32::MIN + 2), &cam), Matrix::of(u32::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MIN + 3, i32::MIN + 3), &cam), Matrix::of(u32::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MAX - 1, i32::MAX - 1), &cam), Matrix::of(1, u32::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MAX - 2, i32::MAX - 2), &cam), Matrix::of(2, u32::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i32::MAX - 3, i32::MAX - 3), &cam), Matrix::of(3, u32::MAX - 3));
    }
}
