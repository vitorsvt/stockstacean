use std::ops::Not;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_color() {
        assert_eq!(Color::White, !Color::Black);
        assert_eq!(Color::Black, !Color::White);
    }
}
