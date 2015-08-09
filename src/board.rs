use std::collections::BTreeSet;
use std::fmt::{Display, Error, Formatter};
use action::{POSSIBLE_DIRECTIONS,POSSIBLE_MOVE_TYPES,Coordinate,Move};

pub const NUM_COLS: usize = 5;
pub const NUM_ROWS: usize = 4;

// Hacks for the three states of the lower-left of the board in JunSeok vs YeonSeung game
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum BoardType {
    Left,
    LeftOrMiddle,
    Middle,
    RightOrMiddle,
    Right,
}

impl BoardType {
    fn is_final(&self) -> bool {
        match *self {
            BoardType::Left => true,
            BoardType::Middle => true,
            BoardType::Right => true,
            BoardType::LeftOrMiddle => false,
            BoardType::RightOrMiddle => false,
        }
    }

    // Can a board of type `current` become this type?
    fn applies_to(&self, current: Option<BoardType>) -> bool {
        match (current, *self) {
            // a none board type can change to anything.
            (None, _) => true,
            // LeftOrMiddle can change to itself, left, or middle.
            (Some(BoardType::LeftOrMiddle), BoardType::LeftOrMiddle) => true,
            (Some(BoardType::LeftOrMiddle), BoardType::Left) => true,
            (Some(BoardType::LeftOrMiddle), BoardType::Middle) => true,
            (Some(BoardType::LeftOrMiddle), _) => false,
            // RightOrMiddle can change to itself, right, or middle.
            (Some(BoardType::RightOrMiddle), BoardType::RightOrMiddle) => true,
            (Some(BoardType::RightOrMiddle), BoardType::Right) => true,
            (Some(BoardType::RightOrMiddle), BoardType::Middle) => true,
            (Some(BoardType::RightOrMiddle), _) => false,
            // Left, Middle, Right can change to themselves only.
            (Some(BoardType::Left), BoardType::Left) => true,
            (Some(BoardType::Left), _) => false,
            (Some(BoardType::Middle), BoardType::Middle) => true,
            (Some(BoardType::Middle), _) => false,
            (Some(BoardType::Right), BoardType::Right) => true,
            (Some(BoardType::Right), _) => false,
        }
    }

    fn induced_by(&self, c: Coordinate) -> bool {
        // Not in the lower left, so it's a free pass.
        if !c.induces_board_type() {
            return true;
        }

        match *self {
            BoardType::Left          => c != Coordinate{row: 2, col: 1} && c != Coordinate{row: 1, col: 1},
            BoardType::LeftOrMiddle  => c == Coordinate{row: 1, col: 0},
            BoardType::Middle        => c != Coordinate{row: 3, col: 0} && c != Coordinate{row: 1, col: 1},
            BoardType::RightOrMiddle => c == Coordinate{row: 3, col: 1},
            BoardType::Right         => c != Coordinate{row: 3, col: 0} && c != Coordinate{row: 2, col: 0},
        }
    }
}

#[derive(Copy, Clone)]
enum Orientation {
    UpDown,
    LeftRight,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Orientation {
    fn to_str(&self) -> &'static str {
        match *self {
            Orientation::UpDown => "║",
            Orientation::LeftRight => "═",
            Orientation::UpLeft => "╝",
            Orientation::UpRight => "╚",
            Orientation::DownLeft => "╗",
            Orientation::DownRight => "╔",
        }
    }
}

#[derive(Copy, Clone)]
enum OrientationOption {
    Fixed(Orientation),
    IfRight(Orientation, Orientation),
    IfLeft(Orientation, Orientation),
    LeftOrMiddle(Orientation, Orientation),
    RightOrMiddle(Orientation, Orientation),
}

impl OrientationOption {
    fn for_board(&self, board_type: Option<BoardType>) -> &'static str {
        let orientation = match (*self, board_type) {
            (OrientationOption::Fixed(orientation), _) => orientation,
            (OrientationOption::IfRight(ifright, _), Some(BoardType::Right)) => ifright,
            (OrientationOption::IfRight(_, notright), _) => notright,
            (OrientationOption::IfLeft(ifleft, _), Some(BoardType::Left)) => ifleft,
            (OrientationOption::IfLeft(_, notleft), _) => notleft,
            (OrientationOption::LeftOrMiddle(ifleft, _), Some(BoardType::Left)) => ifleft,
            (OrientationOption::LeftOrMiddle(_, ifmiddle), Some(BoardType::Middle)) => ifmiddle,
            (OrientationOption::LeftOrMiddle(_, _), _) => unreachable!(),
            (OrientationOption::RightOrMiddle(ifright, _), Some(BoardType::Right)) => ifright,
            (OrientationOption::RightOrMiddle(_, ifmiddle), Some(BoardType::Middle)) => ifmiddle,
            (OrientationOption::RightOrMiddle(_, _), _) => unreachable!(),
        };
        orientation.to_str()
    }
}


const ORIENTATIONS: [[OrientationOption; NUM_COLS]; NUM_ROWS] = [
    [
        OrientationOption::Fixed(Orientation::DownRight),
        OrientationOption::Fixed(Orientation::LeftRight),
        OrientationOption::Fixed(Orientation::LeftRight),
        OrientationOption::Fixed(Orientation::LeftRight),
        OrientationOption::Fixed(Orientation::DownLeft)
    ],
    [
        OrientationOption::IfRight(Orientation::UpRight, Orientation::UpDown),
        OrientationOption::Fixed(Orientation::DownLeft),
        OrientationOption::Fixed(Orientation::DownRight),
        OrientationOption::Fixed(Orientation::LeftRight),
        OrientationOption::Fixed(Orientation::UpLeft)
    ],
    [
        OrientationOption::LeftOrMiddle(Orientation::UpDown, Orientation::UpRight),
        OrientationOption::RightOrMiddle(Orientation::UpDown, Orientation::DownLeft),
        OrientationOption::Fixed(Orientation::UpRight),
        OrientationOption::Fixed(Orientation::LeftRight),
        OrientationOption::Fixed(Orientation::DownLeft)
    ],
    [
        OrientationOption::Fixed(Orientation::UpRight),
        OrientationOption::IfLeft(Orientation::LeftRight, Orientation::UpRight),
        OrientationOption::Fixed(Orientation::LeftRight),
        OrientationOption::Fixed(Orientation::LeftRight),
        OrientationOption::Fixed(Orientation::UpLeft)
    ],
];

const POSSIBLE_BOARD_TYPES: [BoardType; 5] = [
    BoardType::Left,
    BoardType::LeftOrMiddle,
    BoardType::Middle,
    BoardType::RightOrMiddle,
    BoardType::Right,
];

pub type BoardArray = [[bool; NUM_COLS]; NUM_ROWS];
pub struct Board {
    board: BoardArray,
    board_type: Option<BoardType>,
}

impl Board {
    pub fn new(array: BoardArray, board_type: Option<BoardType>) -> Board {
        Board {
            board: array,
            board_type: board_type,
        }
    }

    pub fn make_move(&mut self, m: Move) {
        if let Some(bt) = m.new_board_type {
            if !bt.applies_to(self.board_type) {
                panic!("Board type is {:?}, not compatible with {:?}", self.board_type, bt);
            }
            self.board_type = m.new_board_type
        }
        self.set_squares(m, true)
    }

    pub fn undo_move(&mut self, m: Move) {
        self.board_type = m.old_board_type;
        self.set_squares(m, false)
    }

    fn set_squares(&mut self, m: Move, mode: bool) {
        self.board[m.coord.row][m.coord.col] = mode;
        for other_space in m.extensions().iter() {
            self.board[other_space.row][other_space.col] = mode;
        }
    }

    fn occupied(&self, c: Coordinate) -> bool {
        self.board[c.row][c.col]
    }

    // This assesses whether a coordinate can be placed on the board,
    // given the current type of the board.
    fn compatible(&self, c: Coordinate) -> bool {
        // Not in the lower left, so it's a free pass.
        if !c.induces_board_type() {
            return true;
        }

        match self.board_type {
            Some(BoardType::Left)          => c != Coordinate{row: 2, col: 1} && c != Coordinate{row: 1, col: 1},
            Some(BoardType::LeftOrMiddle)  => c != Coordinate{row: 1, col: 1},
            Some(BoardType::Middle)        => c != Coordinate{row: 3, col: 0} && c != Coordinate{row: 1, col: 1},
            Some(BoardType::RightOrMiddle) => c != Coordinate{row: 3, col: 0},
            Some(BoardType::Right)         => c != Coordinate{row: 3, col: 0} && c != Coordinate{row: 2, col: 0},
            None => true,
        }
    }

    fn frontier(&self) -> Vec<Coordinate> {
        let mut results = Vec::new();
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let coord = Coordinate{row: row, col: col};
                if self.occupied(coord) || !self.compatible(coord) {
                    continue;
                }
                let have_neighbor = POSSIBLE_DIRECTIONS.iter().any(|dir| {
                    coord.move_in(*dir, 1).map_or(false, |x| self.occupied(x))
                });
                if have_neighbor {
                    results.push(coord);
                }
            }
        }
        results
    }

    fn board_type_final(&self) -> bool {
        self.board_type.map_or(false, |x| x.is_final())
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut results = Vec::new();
        for frontier_space in self.frontier().iter() {
            for move_type in POSSIBLE_MOVE_TYPES.iter() {
                let mov = match Move::new(*frontier_space, *move_type, self.board_type) {
                    Some(x) => x,
                    None => continue,
                };
                let mut other_space_taken = false;
                let mut induces_board_type = frontier_space.induces_board_type();
                for other_space in mov.extensions().iter() {
                    if other_space.induces_board_type() {
                        induces_board_type = true;
                    }
                    if self.occupied(*other_space) || !self.compatible(*other_space) {
                        other_space_taken = true;
                        break;
                    }
                }
                if other_space_taken {
                    continue;
                }

                if !induces_board_type || self.board_type_final() {
                    results.push(mov);
                    continue;
                }

                let mut ok_board_types: BTreeSet<_> = POSSIBLE_BOARD_TYPES.iter().cloned().filter(|board_type| {
                    board_type.applies_to(self.board_type) &&
                        board_type.induced_by(*frontier_space) &&
                        mov.extensions().iter().all(|coord| board_type.induced_by(*coord))
                }).collect();

                // Dominated board types...
                if ok_board_types.contains(&BoardType::LeftOrMiddle) {
                    ok_board_types.remove(&BoardType::Left);
                    ok_board_types.remove(&BoardType::Middle);
                }
                if ok_board_types.contains(&BoardType::RightOrMiddle) {
                    ok_board_types.remove(&BoardType::Right);
                    ok_board_types.remove(&BoardType::Middle);
                }

                for board_type in ok_board_types.iter() {
                    results.push(mov.with_board_type(*board_type));
                }
            }
        }
        results
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        // Print header row
        try!(formatter.write_str("    "));
        for i in 0..NUM_COLS {
            try!(write!(formatter, " {}", i));
        }
        try!(formatter.write_str("\n"));

        // Print top box border
        try!(formatter.write_str("    ┌─"));
        for _ in 0..NUM_COLS - 1 {
            try!(formatter.write_str("┬─"));
        }
        try!(formatter.write_str("┐\n"));

        for (i, row) in self.board.iter().enumerate() {
            // Print cell content
            try!(write!(formatter, "{: >2}  │", i));
            for (j, col) in row.iter().enumerate() {
                let chr = if *col { ORIENTATIONS[i][j].for_board(self.board_type) } else { " " };
                try!(write!(formatter, "{}│", chr));
            }
            try!(formatter.write_str("\n"));

            // Print box border between rows
            if i != NUM_ROWS - 1 {
                try!(formatter.write_str("    ├─"));
                for _ in 0..NUM_COLS - 1 {
                    try!(formatter.write_str("┼─"));
                }
                try!(formatter.write_str("┤\n"));
            }
        }

        // Print bottom box border
        try!(formatter.write_str("    └─"));
        for _ in 0..NUM_COLS - 1 {
            try!(formatter.write_str("┴─"));
        }
        formatter.write_str("┘\n")
    }
}

#[cfg(test)]
mod tests {
    use super::{Board,BoardArray,BoardType};
    use action::Coordinate;

    const START_BOARD: BoardArray = [
        [false,  true,  true,  true, false],
        [false, false, false,  true, false],
        [false, false, false,  true, false],
        [false, false, false, false, false],
    ];

    const LEFT_BOARD_FROM_TOP: BoardArray = [
        [ true,  true,  true,  true, false],
        [ true, false, false,  true, false],
        [ true, false, false,  true, false],
        [false, false,  true,  true, false],
    ];

    const LEFT_BOARD_FROM_BOTTOM: BoardArray = [
        [ true,  true,  true,  true, false],
        [false, false, false,  true, false],
        [false, false, false,  true, false],
        [ true,  true,  true,  true, false],
    ];

    const MIDDLE_BOARD_FROM_LEFT: BoardArray = [
        [ true,  true,  true,  true, false],
        [ true, false, false,  true, false],
        [ true, false, false,  true, false],
        [false, false,  true,  true, false],
    ];

    const MIDDLE_BOARD_FROM_RIGHT: BoardArray = [
        [ true,  true,  true,  true, false],
        [false, false, false,  true, false],
        [false,  true,  true,  true, false],
        [false, false, false, false, false],
    ];

    const RIGHT_BOARD_FROM_TOP: BoardArray = [
        [ true,  true,  true,  true, false],
        [false,  true, false,  true, false],
        [false, false,  true,  true, false],
        [false, false,  true,  true, false],
    ];

    const RIGHT_BOARD_FROM_BOTTOM: BoardArray = [
        [ true,  true,  true,  true, false],
        [false, false, false,  true, false],
        [false,  true, false,  true, false],
        [false,  true,  true,  true, false],
    ];

    const LEFT_OR_MIDDLE_BOARD: BoardArray = [
        [ true,  true,  true,  true, false],
        [ true, false, false,  true, false],
        [false, false,  true,  true, false],
        [false, false, false, false, false],
    ];

    const RIGHT_OR_MIDDLE_BOARD: BoardArray = [
        [ true,  true,  true,  true, false],
        [false, false, false,  true, false],
        [false, false, false,  true, false],
        [false,  true,  true,  true, false],
    ];

    const FINISHED_LEFT_BOARD: BoardArray = [
        [ true,  true,  true,  true,  true],
        [ true, false,  true,  true,  true],
        [ true, false,  true,  true,  true],
        [ true,  true,  true,  true,  true],
    ];

    const FINISHED_MIDDLE_BOARD: BoardArray = [
        [ true,  true,  true,  true,  true],
        [ true, false,  true,  true,  true],
        [ true,  true,  true,  true,  true],
        [false,  true,  true,  true,  true],
    ];

    const FINISHED_RIGHT_BOARD: BoardArray = [
        [ true,  true,  true,  true,  true],
        [ true,  true,  true,  true,  true],
        [false,  true,  true,  true,  true],
        [false,  true,  true,  true,  true],
    ];

    #[test]
    fn start_board_allows_left_or_middle() {
        let board = Board::new(START_BOARD, None);
        let moves: Vec<_> = board.legal_moves().iter().cloned().filter(|mv| mv.new_board_type == Some(BoardType::LeftOrMiddle)).collect();
        // This is a questionable test.
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn start_board_allows_right_or_middle() {
        let board = Board::new(START_BOARD, None);
        let moves: Vec<_> = board.legal_moves().iter().cloned().filter(|mv| mv.new_board_type == Some(BoardType::RightOrMiddle)).collect();
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn left_board_from_top_allows_left_move() {
        let board = Board::new(LEFT_BOARD_FROM_TOP, Some(BoardType::Left));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 0 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 1 }));
    }

    #[test]
    fn left_board_from_top_forbids_non_left_move() {
        let board = Board::new(LEFT_BOARD_FROM_TOP, Some(BoardType::Left));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 1 }));
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 1 }));
    }

    #[test]
    fn left_board_from_bottom_allows_left_move() {
        let board = Board::new(LEFT_BOARD_FROM_BOTTOM, Some(BoardType::Left));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 0 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 0 }));
    }

    #[test]
    fn left_board_from_bottom_forbids_non_left_move() {
        let board = Board::new(LEFT_BOARD_FROM_BOTTOM, Some(BoardType::Left));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 1 }));
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 1 }));
    }

    #[test]
    fn middle_board_from_left_allows_middle_move() {
        let board = Board::new(MIDDLE_BOARD_FROM_LEFT, Some(BoardType::Middle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 1 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 1 }));
    }

    #[test]
    fn middle_board_from_left_forbids_non_middle_move() {
        let board = Board::new(MIDDLE_BOARD_FROM_LEFT, Some(BoardType::Middle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 1 }));
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 0 }));
    }

    #[test]
    fn middle_board_from_right_allows_middle_move() {
        let board = Board::new(MIDDLE_BOARD_FROM_RIGHT, Some(BoardType::Middle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 0 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 0 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 1 }));
    }

    #[test]
    fn middle_board_from_right_forbids_non_middle_move() {
        let board = Board::new(MIDDLE_BOARD_FROM_RIGHT, Some(BoardType::Middle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 1 }));
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 0 }));
    }

    #[test]
    fn right_board_from_top_allows_right_move() {
        let board = Board::new(RIGHT_BOARD_FROM_TOP, Some(BoardType::Right));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 1 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 1 }));
    }

    #[test]
    fn right_board_from_top_forbids_non_right_move() {
        let board = Board::new(RIGHT_BOARD_FROM_TOP, Some(BoardType::Right));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 0 }));
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 0 }));
    }

    #[test]
    fn right_board_from_bottom_allows_right_move() {
        let board = Board::new(RIGHT_BOARD_FROM_BOTTOM, Some(BoardType::Right));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 1 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 0 }));
    }

    #[test]
    fn right_board_from_bottom_forbids_non_right_move() {
        let board = Board::new(RIGHT_BOARD_FROM_BOTTOM, Some(BoardType::Right));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 0 }));
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 0 }));
    }

    #[test]
    fn left_or_middle_board_allows_left_move() {
        let board = Board::new(LEFT_OR_MIDDLE_BOARD, Some(BoardType::LeftOrMiddle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        let move_types: Vec<_> = board.legal_moves().iter().map(|mv| mv.new_board_type).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 0 }));
        assert!(move_types.iter().any(|mv| *mv == Some(BoardType::Left)));
    }

    #[test]
    fn left_or_middle_board_allows_middle_move() {
        let board = Board::new(LEFT_OR_MIDDLE_BOARD, Some(BoardType::LeftOrMiddle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        let move_types: Vec<_> = board.legal_moves().iter().map(|mv| mv.new_board_type).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 1 }));
        assert!(move_types.iter().any(|mv| *mv == Some(BoardType::Middle)));
    }

    #[test]
    fn left_or_middle_board_forbids_right_move() {
        let board = Board::new(LEFT_OR_MIDDLE_BOARD, Some(BoardType::LeftOrMiddle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        let move_types: Vec<_> = board.legal_moves().iter().map(|mv| mv.new_board_type).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 1 }));
        assert!(!move_types.iter().any(|mv| *mv == Some(BoardType::Right)));
    }

    #[test]
    fn right_or_middle_board_allows_right_move() {
        let board = Board::new(RIGHT_OR_MIDDLE_BOARD, Some(BoardType::RightOrMiddle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        let move_types: Vec<_> = board.legal_moves().iter().map(|mv| mv.new_board_type).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 1, col: 1 }));
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 1 }));
        assert!(move_types.iter().any(|mv| *mv == Some(BoardType::Right)));
    }

    #[test]
    fn right_or_middle_board_allows_middle_move() {
        let board = Board::new(RIGHT_OR_MIDDLE_BOARD, Some(BoardType::RightOrMiddle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        let move_types: Vec<_> = board.legal_moves().iter().map(|mv| mv.new_board_type).collect();
        assert!(move_coords.iter().any(|mv| *mv == Coordinate { row: 2, col: 1 }));
        assert!(move_types.iter().any(|mv| *mv == Some(BoardType::Middle)));
    }

    #[test]
    fn right_or_middle_board_forbids_left_move() {
        let board = Board::new(RIGHT_OR_MIDDLE_BOARD, Some(BoardType::RightOrMiddle));
        let move_coords: Vec<_> = board.legal_moves().iter().map(|mv| mv.coord).collect();
        let move_types: Vec<_> = board.legal_moves().iter().map(|mv| mv.new_board_type).collect();
        assert!(!move_coords.iter().any(|mv| *mv == Coordinate { row: 3, col: 0 }));
        assert!(!move_types.iter().any(|mv| *mv == Some(BoardType::Left)));
    }

    #[test]
    fn finished_left_board_has_no_moves() {
        let board = Board::new(FINISHED_LEFT_BOARD, Some(BoardType::Left));
        assert!(board.legal_moves().is_empty());
    }

    #[test]
    fn finished_middle_board_has_no_moves() {
        let board = Board::new(FINISHED_MIDDLE_BOARD, Some(BoardType::Middle));
        assert!(board.legal_moves().is_empty());
    }

    #[test]
    fn finished_right_board_has_no_moves() {
        let board = Board::new(FINISHED_RIGHT_BOARD, Some(BoardType::Right));
        assert!(board.legal_moves().is_empty());
    }
}
