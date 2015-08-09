use std::fmt::{Display, Error, Formatter};
use board;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn move_in(&self, dir: Direction, delta: usize) -> Option<Coordinate> {
        match dir {
            Direction::Up => if self.row >= delta { Some(Coordinate{row: self.row - delta, col: self.col}) } else { None },
            Direction::Down => if self.row + delta < board::NUM_ROWS { Some(Coordinate{row: self.row + delta, col: self.col}) } else { None },
            Direction::Left => if self.col >= delta { Some(Coordinate{row: self.row, col: self.col - delta}) } else { None },
            Direction::Right => if self.col + delta < board::NUM_COLS { Some(Coordinate{row: self.row, col: self.col + delta}) } else { None },
        }
    }

    fn unchecked_move_in(&self, dir: Direction, delta: usize) -> Coordinate {
        match dir {
            Direction::Up => Coordinate{row: self.row - delta, col: self.col},
            Direction::Down => Coordinate{row: self.row + delta, col: self.col},
            Direction::Left => Coordinate{row: self.row, col: self.col - delta},
            Direction::Right => Coordinate{row: self.row, col: self.col + delta},
        }
    }

    pub fn induces_board_type(&self) -> bool {
        // The lower left corner of the board.
        self.col < 2 && self.row >= 1
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub const POSSIBLE_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Copy, Clone, Debug)]
pub enum MoveType {
    Single,
    OneUp,
    OneDown,
    OneLeft,
    OneRight,
    TwoUp,
    TwoDown,
    TwoLeft,
    TwoRight,
    UpAndDown,
    LeftAndRight,
}
pub const POSSIBLE_MOVE_TYPES: [MoveType; 11] = [
    MoveType::Single,
    MoveType::OneUp,
    MoveType::OneDown,
    MoveType::OneLeft,
    MoveType::OneRight,
    MoveType::TwoUp,
    MoveType::TwoDown,
    MoveType::TwoLeft,
    MoveType::TwoRight,
    MoveType::UpAndDown,
    MoveType::LeftAndRight,
];

#[derive(Copy, Clone)]
pub struct Move {
    pub coord: Coordinate,
    move_type: MoveType,
    pub old_board_type: Option<board::BoardType>,
    pub new_board_type: Option<board::BoardType>,
}

impl Move {
    pub fn new(coord: Coordinate, move_type: MoveType, old_board_type: Option<board::BoardType>) -> Option<Move> {
        let c = Move{coord: coord, move_type: move_type, old_board_type: old_board_type, new_board_type: None};
        if c.in_bounds() { Some(c) } else { None }
    }

    pub fn with_board_type(&self, new_board_type: board::BoardType) -> Move {
        Move{new_board_type: Some(new_board_type), .. *self}
    }

    pub fn in_bounds(&self) -> bool {
        match self.move_type {
            MoveType::Single => true,
            MoveType::OneUp => self.coord.row >= 1,
            MoveType::OneDown => self.coord.row < board::NUM_ROWS - 1,
            MoveType::OneLeft => self.coord.col >= 1,
            MoveType::OneRight => self.coord.col < board::NUM_COLS - 1,
            MoveType::TwoUp => self.coord.row >= 2,
            MoveType::TwoDown => self.coord.row < board::NUM_ROWS - 2,
            MoveType::TwoLeft => self.coord.col >= 2,
            MoveType::TwoRight => self.coord.col < board::NUM_COLS - 2,
            MoveType::UpAndDown => self.coord.row >= 1 && self.coord.row < board::NUM_ROWS - 1,
            MoveType::LeftAndRight => self.coord.col >= 1 && self.coord.col < board::NUM_COLS - 1,
        }
    }

    pub fn extensions(&self) -> Vec<Coordinate> {
        match self.move_type {
            MoveType::Single => vec![],
            MoveType::OneUp => vec![self.coord.unchecked_move_in(Direction::Up, 1)],
            MoveType::OneDown => vec![self.coord.unchecked_move_in(Direction::Down, 1)],
            MoveType::OneLeft => vec![self.coord.unchecked_move_in(Direction::Left, 1)],
            MoveType::OneRight => vec![self.coord.unchecked_move_in(Direction::Right, 1)],
            MoveType::TwoUp => vec![self.coord.unchecked_move_in(Direction::Up, 1), self.coord.unchecked_move_in(Direction::Up, 2)],
            MoveType::TwoDown => vec![self.coord.unchecked_move_in(Direction::Down, 1), self.coord.unchecked_move_in(Direction::Down, 2)],
            MoveType::TwoLeft => vec![self.coord.unchecked_move_in(Direction::Left, 1), self.coord.unchecked_move_in(Direction::Left, 2)],
            MoveType::TwoRight => vec![self.coord.unchecked_move_in(Direction::Right, 1), self.coord.unchecked_move_in(Direction::Right, 2)],
            MoveType::UpAndDown => vec![self.coord.unchecked_move_in(Direction::Up, 1), self.coord.unchecked_move_in(Direction::Down, 1)],
            MoveType::LeftAndRight => vec![self.coord.unchecked_move_in(Direction::Left, 1), self.coord.unchecked_move_in(Direction::Right, 1)],
        }
    }
}

impl Display for Move {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self.new_board_type {
            Some(bt) => write!(formatter, "{:?} at {:?} (Board type {:?})", self.move_type, self.coord, bt),
            None => write!(formatter, "{:?} at {:?}", self.move_type, self.coord),
        }
    }
}
