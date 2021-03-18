/// Ranks (horizontal lines) of the board
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    /// The rank above. Wraps around.
    pub fn up(&self) -> Rank {
        Rank::from(*self as usize + 1)
    }

    /// The rank below. Wraps around.
    pub fn down(&self) -> Rank {
        Rank::from((*self as usize).wrapping_sub(1))
    }
}

impl From<usize> for Rank {
    fn from(number: usize) -> Rank {
        match number % 8 {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            _ => Rank::Eighth,
        }
    }
}

#[test]
pub fn test_conversions() {
    assert_eq!(Rank::First as usize, 0);
    assert_eq!(Rank::First, Rank::from(0));
}

#[test]
pub fn test_rank_above_and_below() {
    assert_eq!(Rank::First.up(), Rank::Second);
    assert_eq!(Rank::Eighth.down(), Rank::Seventh);
    assert_eq!(Rank::First.down(), Rank::Eighth);
    assert_eq!(Rank::Eighth.up(), Rank::First);
}
