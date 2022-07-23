use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub index: usize,
    pub collapsed: bool,
    pub options: Vec<TileKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileKind {
    Blank,
    Up,
    Right,
    Down,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Vertical,
    Horizontal,
    UpEnd,
    RightEnd,
    DownEnd,
    LeftEnd,
    Cross,
}

impl TileKind {
    pub fn sides(&self) -> [usize; 4] {
        match self {
            Self::Blank => [0, 0, 0, 0],
            Self::Up => [1, 1, 0, 1],
            Self::Right => [1, 1, 1, 0],
            Self::Down => [0, 1, 1, 1],
            Self::Left => [1, 0, 1, 1],
            Self::UpRight => [1, 1, 0, 0],
            Self::UpLeft => [1, 0, 0, 1],
            Self::DownRight => [0, 1, 1, 0],
            Self::DownLeft => [0, 0, 1, 1],
            Self::Vertical => [1, 0, 1, 0],
            Self::Horizontal => [0, 1, 0, 1],
            Self::UpEnd => [1, 0, 0, 0],
            Self::RightEnd => [0, 1, 0, 0],
            Self::DownEnd => [0, 0, 1, 0],
            Self::LeftEnd => [0, 0, 0, 1],
            Self::Cross => [1, 1, 1, 1],
        }
    }
    pub fn fit(&self, sides: &[Option<usize>; 4]) -> bool {
        for (i, x) in self.sides().iter().enumerate() {
            if let Some(side) = sides[i] {
                if !x.eq(&side) {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for TileKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blank => " ",
                Self::Up => "┻",
                Self::Right => "┣",
                Self::Down => "┳",
                Self::Left => "┫",
                Self::UpRight => "┗",
                Self::UpLeft => "┛",
                Self::DownRight => "┏",
                Self::DownLeft => "┓",
                Self::Vertical => "┃",
                Self::Horizontal => "━",
                Self::UpEnd => "╹",
                Self::RightEnd => "╺",
                Self::DownEnd => "╻",
                Self::LeftEnd => "╸",
                Self::Cross => "╋",
            }
        )
    }
}
