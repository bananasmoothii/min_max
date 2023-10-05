use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
pub enum CountDirection {
    HorizontalRight,
    DiagonalDownRight,
    VerticalDown,
    DiagonalDownLeft,
    HorizontalLeft,
    DiagonalUpLeft,
    VerticalUp,
    DiagonalUpRight,
}

impl CountDirection {
    pub fn add_to(&self, coords: (usize, usize)) -> Option<(usize, usize)> {
        let (y, x) = coords;
        match self {
            CountDirection::HorizontalRight => {
                if x < 6 {
                    Some((y, x + 1))
                } else {
                    None
                }
            }
            CountDirection::DiagonalDownRight => {
                if x < 6 && y < 5 {
                    Some((y + 1, x + 1))
                } else {
                    None
                }
            }
            CountDirection::VerticalDown => {
                if y < 5 {
                    Some((y + 1, x))
                } else {
                    None
                }
            }
            CountDirection::DiagonalDownLeft => {
                if x > 0 && y < 5 {
                    Some((y + 1, x - 1))
                } else {
                    None
                }
            }
            CountDirection::HorizontalLeft => {
                if x > 0 {
                    Some((y, x - 1))
                } else {
                    None
                }
            }
            CountDirection::DiagonalUpLeft => {
                if x > 0 && y > 0 {
                    Some((y - 1, x - 1))
                } else {
                    None
                }
            }
            CountDirection::VerticalUp => {
                if y > 0 {
                    Some((y - 1, x))
                } else {
                    None
                }
            }
            CountDirection::DiagonalUpRight => {
                if x < 6 && y > 0 {
                    Some((y - 1, x + 1))
                } else {
                    None
                }
            }
        }
    }

    pub fn opposite(&self) -> CountDirection {
        match self {
            CountDirection::HorizontalRight => CountDirection::HorizontalLeft,
            CountDirection::DiagonalDownRight => CountDirection::DiagonalUpLeft,
            CountDirection::VerticalDown => CountDirection::VerticalUp,
            CountDirection::DiagonalDownLeft => CountDirection::DiagonalUpRight,
            CountDirection::HorizontalLeft => CountDirection::HorizontalRight,
            CountDirection::DiagonalUpLeft => CountDirection::DiagonalDownRight,
            CountDirection::VerticalUp => CountDirection::VerticalDown,
            CountDirection::DiagonalUpRight => CountDirection::DiagonalDownLeft,
        }
    }

    pub fn half_side() -> [CountDirection; 4] {
        [
            CountDirection::HorizontalRight,
            CountDirection::DiagonalDownRight,
            CountDirection::VerticalDown,
            CountDirection::DiagonalDownLeft,
        ]
    }
}
