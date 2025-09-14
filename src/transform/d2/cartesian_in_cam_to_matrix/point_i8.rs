type Cartesian = crate::cartesian::d2::point::point_i8::Point;
type Matrix = crate::matrix::d2::point::point_u8::Point;
type Cam = crate::cartesian::d2::rect::rect_i8::Rect;

pub fn cartesian_in_cam_to_matrix(point: &Cartesian, cam: &Cam) -> Matrix {
    let row = i16::from(cam.max.y) - i16::from(point.y);
    let col = i16::from(point.x) - i16::from(cam.min.x);
    Matrix { row: row as u8, col: col as u8 }
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
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::min(), &cam), Matrix::of(u8::MAX, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MAX, i8::MIN), &cam), Matrix::max());
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MIN, i8::MAX), &cam), Matrix::of(0, 0));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::max(), &cam), Matrix::of(0, u8::MAX));
    }

    #[test]
    fn sequence_min() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MIN + 1, i8::MIN + 1), &cam), Matrix::of(u8::MAX - 1, 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MIN + 2, i8::MIN + 2), &cam), Matrix::of(u8::MAX - 2, 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MIN + 3, i8::MIN + 3), &cam), Matrix::of(u8::MAX - 3, 3));
    }

    #[test]
    fn sequence_max() {
        let cam = Cam::largest();
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MAX - 1, i8::MAX - 1), &cam), Matrix::of(1, u8::MAX - 1));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MAX - 2, i8::MAX - 2), &cam), Matrix::of(2, u8::MAX - 2));
        assert_eq!(cartesian_in_cam_to_matrix(&Cartesian::of(i8::MAX - 3, i8::MAX - 3), &cam), Matrix::of(3, u8::MAX - 3));
    }
}
