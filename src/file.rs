// Files (vertical lines) of the board
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    // The file to the right. Wraps around.
    pub fn right(&self) -> File {
        File::from(*self as usize + 1)
    }

    // The file to the left. Wraps around.
    pub fn left(&self) -> File {
        File::from((*self as usize).wrapping_sub(1))
    }
}

impl From<usize> for File {
    fn from(number: usize) -> File {
        match number % 8 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            _ => File::H,
        }
    }
}

#[test]
pub fn test_conversions() {
    assert_eq!(File::A as usize, 0);
    assert_eq!(File::A, File::from(0));
}

#[test]
pub fn test_file_left_and_right() {
    assert_eq!(File::A.right(), File::B);
    assert_eq!(File::H.left(), File::G);
    assert_eq!(File::A.left(), File::H);
    assert_eq!(File::H.right(), File::A);
}
