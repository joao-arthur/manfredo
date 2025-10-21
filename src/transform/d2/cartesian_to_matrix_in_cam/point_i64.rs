type Cartesian = crate::cartesian::d2::point::point_i64::Point;
type Matrix = crate::matrix::d2::point::point_u64::Point;
type Cam = crate::matrix::d2::rect::rect_u64::Rect;

pub fn cartesian_to_matrix_in_cam(point: &Cartesian, cam: &Cam) -> Matrix {
    let row = i128::from(i64::MAX) - i128::from(point.y) + i128::from(cam.min.row);
    let col = i128::from(point.x) - i128::from(i64::MIN) + i128::from(cam.min.col);
    Matrix { row: row as u64, col: col as u64 }
}

#[cfg(test)]
mod tests {
    use super::{Cam, Cartesian, Matrix, cartesian_to_matrix_in_cam};

    #[test]
    fn test_3x3() {
        let cam = Cam::of(0, 0, 2, 2);
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX - 2), &cam), Matrix::of(2, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX - 1), &cam), Matrix::of(1, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX), &cam), Matrix::min());

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MAX - 2), &cam), Matrix::of(2, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MAX - 1), &cam), Matrix::of(1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MAX), &cam), Matrix::of(0, 1));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MAX - 2), &cam), Matrix::of(2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MAX - 1), &cam), Matrix::of(1, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MAX), &cam), Matrix::of(0, 2));
    }

    #[test]
    fn test_4x4() {
        let cam = Cam::of(10, 10, 13, 13);
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX - 3), &cam), Matrix::of(13, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX - 2), &cam), Matrix::of(12, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX - 1), &cam), Matrix::of(11, 10));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX), &cam), Matrix::of(10, 10));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MAX - 3), &cam), Matrix::of(13, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MAX - 2), &cam), Matrix::of(12, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MAX - 1), &cam), Matrix::of(11, 11));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MAX), &cam), Matrix::of(10, 11));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MAX - 3), &cam), Matrix::of(13, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MAX - 2), &cam), Matrix::of(12, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MAX - 1), &cam), Matrix::of(11, 12));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MAX), &cam), Matrix::of(10, 12));

        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 3, i64::MAX - 3), &cam), Matrix::of(13, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 3, i64::MAX - 2), &cam), Matrix::of(12, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 3, i64::MAX - 1), &cam), Matrix::of(11, 13));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 3, i64::MAX), &cam), Matrix::of(10, 13));
    }

    #[test]
    fn bounds() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::min(), &cam), Matrix::of(u64::MAX, 0));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MAX, i64::MIN), &cam), Matrix::max());
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN, i64::MAX), &cam), Matrix::min());
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::max(), &cam), Matrix::of(0, u64::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 1, i64::MIN + 1), &cam), Matrix::of(u64::MAX - 1, 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 2, i64::MIN + 2), &cam), Matrix::of(u64::MAX - 2, 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MIN + 3, i64::MIN + 3), &cam), Matrix::of(u64::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MAX - 1, i64::MAX - 1), &cam), Matrix::of(1, u64::MAX - 1));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MAX - 2, i64::MAX - 2), &cam), Matrix::of(2, u64::MAX - 2));
        assert_eq!(cartesian_to_matrix_in_cam(&Cartesian::of(i64::MAX - 3, i64::MAX - 3), &cam), Matrix::of(3, u64::MAX - 3));
    }
}
