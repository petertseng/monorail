#[derive(Copy, Clone, Debug)]
enum Player {
    YeonSeung,
    JunSeok,
}

impl Player {
    fn opponent(&self) -> Player {
        match *self {
            Player::YeonSeung => Player::JunSeok,
            Player::JunSeok => Player::YeonSeung,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn move_in(&self, dir: Direction, delta: usize) -> Coordinate {
        match dir {
            Direction::Up => Coordinate{row: self.row - delta, col: self.col},
            Direction::Down => Coordinate{row: self.row + delta, col: self.col},
            Direction::Left => Coordinate{row: self.row, col: self.col - delta},
            Direction::Right => Coordinate{row: self.row, col: self.col + delta},
        }
    }
    fn induces_board_type(&self) -> bool {
        *self == Coordinate{row: 1, col: 1} ||
        *self == Coordinate{row: 2, col: 1} ||
        *self == Coordinate{row: 2, col: 0} ||
        *self == Coordinate{row: 3, col: 0}
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
const POSSIBLE_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Copy, Clone, Debug)]
enum MoveType {
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
const POSSIBLE_MOVE_TYPES: [MoveType; 11] = [
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

#[derive(Copy, Clone, Debug)]
struct Move {
    coord: Coordinate,
    move_type: MoveType,
    board_type: Option<BoardType>,
}
impl Move {
    fn coords(&self) -> Vec<Coordinate> {
        match self.move_type {
            MoveType::Single => vec![],
            MoveType::OneUp => vec![self.coord.move_in(Direction::Up, 1)],
            MoveType::OneDown => vec![self.coord.move_in(Direction::Down, 1)],
            MoveType::OneLeft => vec![self.coord.move_in(Direction::Left, 1)],
            MoveType::OneRight => vec![self.coord.move_in(Direction::Right, 1)],
            MoveType::TwoUp => vec![self.coord.move_in(Direction::Up, 1), self.coord.move_in(Direction::Up, 2)],
            MoveType::TwoDown => vec![self.coord.move_in(Direction::Down, 1), self.coord.move_in(Direction::Down, 2)],
            MoveType::TwoLeft => vec![self.coord.move_in(Direction::Left, 1), self.coord.move_in(Direction::Left, 2)],
            MoveType::TwoRight => vec![self.coord.move_in(Direction::Right, 1), self.coord.move_in(Direction::Right, 2)],
            MoveType::UpAndDown => vec![self.coord.move_in(Direction::Up, 1), self.coord.move_in(Direction::Down, 1)],
            MoveType::LeftAndRight => vec![self.coord.move_in(Direction::Left, 1), self.coord.move_in(Direction::Right, 1)],
        }
    }
}

const NUM_COLS: usize = 5;
const NUM_ROWS: usize = 4;

// Hacks for the three states of the lower-left of the board in JunSeok vs YeonSeung game
#[derive(Copy, Clone, Debug)]
enum BoardType {
    Left,
    Middle,
    Right,
}
const POSSIBLE_BOARD_TYPES: [BoardType; 3] = [
    BoardType::Left,
    BoardType::Middle,
    BoardType::Right,
];

type BoardArray = [[bool; NUM_COLS]; NUM_ROWS];
struct Board {
    board: BoardArray,
    board_type: Option<BoardType>,
}

impl Board {
    fn make_move(&mut self, m: Move) {
        match m.board_type {
            Some(_) => self.board_type = m.board_type,
            None => (),
        }
        self.set_squares(m, true)
    }

    fn undo_move(&mut self, m: Move, bt: Option<BoardType>) {
        self.board_type = bt;
        self.set_squares(m, false)
    }

    fn set_squares(&mut self, m: Move, mode: bool) {
        self.board[m.coord.row][m.coord.col] = mode;
        for other_space in m.coords().iter() {
            self.board[other_space.row][other_space.col] = mode;
        }
    }

    fn occupied(&self, c: Coordinate) -> bool {
        self.in_bounds(c) && self.board[c.row][c.col]
    }

    fn in_bounds(&self, c: Coordinate) -> bool {
        // >= 0 is always true due to type limits.
        // c.row >= 0 && c.row < NUM_ROWS && c.col >= 0 && c.col < NUM_COLS
        c.row < NUM_ROWS && c.col < NUM_COLS
    }

    // Assuming that m is a move with an unoccupied coordinate!
    // This doesn't check whether the target squares are occupied.
    // Advantage: It's quicker. Disadvantage: It allows some illegal moves.
    fn move_in_bounds(&self, m: Move) -> bool {
        match m.move_type {
            MoveType::Single => true,
            MoveType::OneUp => m.coord.row >= 1,
            MoveType::OneDown => m.coord.row < NUM_ROWS - 1,
            MoveType::OneLeft => m.coord.col >= 1,
            MoveType::OneRight => m.coord.col < NUM_COLS - 1,
            MoveType::TwoUp => m.coord.row >= 2,
            MoveType::TwoDown => m.coord.row < NUM_ROWS - 2,
            MoveType::TwoLeft => m.coord.col >= 2,
            MoveType::TwoRight => m.coord.col < NUM_COLS - 2,
            MoveType::UpAndDown => m.coord.row >= 1 && m.coord.row < NUM_ROWS - 1,
            MoveType::LeftAndRight => m.coord.col >= 1 && m.coord.col < NUM_COLS - 1,
        }
    }

    // Unfortunately I kind of have to hard-code this.
    fn makes_unsolvable(c: Coordinate, b: Option<BoardType>) -> bool {
        match b {
            Some(BoardType::Left)   => c == Coordinate{row: 2, col: 1} || c == Coordinate{row: 1, col: 1},
            Some(BoardType::Middle) => c == Coordinate{row: 3, col: 0} || c == Coordinate{row: 1, col: 1},
            Some(BoardType::Right)  => c == Coordinate{row: 3, col: 0} || c == Coordinate{row: 2, col: 0},
            None => false,
        }
    }

    fn frontier(&self) -> Vec<Coordinate> {
        let mut results = Vec::new();
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let coord = Coordinate{row: row, col: col};
                if self.occupied(coord) || Board::makes_unsolvable(coord, self.board_type) {
                    continue;
                }
                let mut have_neighbor = false;
                for dir in POSSIBLE_DIRECTIONS.iter() {
                    let new_coord = coord.move_in(*dir, 1);
                    if self.occupied(new_coord) {
                        have_neighbor = true;
                        break;
                    }
                }
                if have_neighbor {
                    results.push(coord);
                }
            }
        }
        results
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut results = Vec::new();
        for frontier_space in self.frontier().iter() {
            for move_type in POSSIBLE_MOVE_TYPES.iter() {
                let mov = Move{coord: *frontier_space, move_type: *move_type, board_type: None};
                if !self.move_in_bounds(mov) {
                    continue;
                }
                let mut other_space_taken = false;
                let mut induces_board_type = frontier_space.induces_board_type();
                for other_space in mov.coords().iter() {
                    if other_space.induces_board_type() {
                        induces_board_type = true;
                    }
                    if self.occupied(*other_space) {
                        other_space_taken = true;
                        break;
                    }
                }
                if !other_space_taken {
                    if induces_board_type && self.board_type.is_none() {
                        for board_type in POSSIBLE_BOARD_TYPES.iter() {
                            if Board::makes_unsolvable(*frontier_space, Some(*board_type)) {
                                continue;
                            }
                            let mut other_spaces_ok = true;
                            for other_space in mov.coords().iter() {
                                if Board::makes_unsolvable(*other_space, Some(*board_type)) {
                                    other_spaces_ok = false;
                                    break;
                                }
                            }
                            if other_spaces_ok {
                                results.push(Move{coord: mov.coord, move_type: mov.move_type, board_type: Some(*board_type)});
                            }
                        }

                    } else {
                        results.push(mov);
                    }
                }
            }
        }
        results
    }

    fn print(&self) {
        for row in self.board.iter() {
            for col in row.iter() {
                print!("{: >5} ", col);
            }
            println!("");
        }
        println!("{:?}", self.board_type);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum GameResult {
    PlaceholderJunSeok,
    JunSeokWin,
    YeonSeungWin,
    PlaceholderYeonSeung,
}

fn minimax_alpha_beta(player: Player, board: &mut Board, initial_alpha: GameResult, initial_beta: GameResult) -> (GameResult, Option<Move>) {
    let moves = board.legal_moves();
    // There are no more moves, which means my opponent completed the railroad.
    // So I lose.
    if moves.is_empty() {
        return match player {
            Player::YeonSeung => (GameResult::JunSeokWin, None),
            Player::JunSeok => (GameResult::YeonSeungWin, None),
        }
    }
    let mut best = match player {
        Player::YeonSeung => initial_alpha,
        Player::JunSeok => initial_beta,
    };
    let mut alpha = initial_alpha;
    let mut beta = initial_beta;
    let mut best_move = None;

    for possible_move in moves.iter() {
        let bt = board.board_type;
        board.make_move(*possible_move);
        let (reply, _) = minimax_alpha_beta(player.opponent(), board, alpha, beta);
        board.undo_move(*possible_move, bt);

        match player {
            Player::YeonSeung => {
                if reply > best {
                    best = reply;
                    alpha = reply;
                    best_move = Some(*possible_move);
                }
                if best >= GameResult::YeonSeungWin {
                    return (best, best_move);
                }
            },
            Player::JunSeok => {
                if reply < best {
                    best = reply;
                    beta = reply;
                    best_move = Some(*possible_move);
                }
                if best <= GameResult::JunSeokWin {
                    return (best, best_move);
                }
            },
        }

        if alpha >= beta {
            return (best, best_move);
        }
    }

    (best, best_move)
}

fn main() {
    let mut starting_board = Board{
        board: [
            [false,  true,  true,  true, false],
            [false, false, false,  true, false],
            [false, false, false,  true, false],
            [false, false, false, false, false],
        ],
        board_type: None,
    };
    let starting_player = Player::YeonSeung;

    let all_responses = false;
    let best_move = false;
    let legal_moves = false;
    let interactive = true;

    if legal_moves {
        for legal_move in starting_board.legal_moves().iter() {
            println!("{:?}", legal_move);
        }
    }

    if best_move {
        let (result, best_move) = minimax_alpha_beta(starting_player, &mut starting_board, GameResult::PlaceholderJunSeok, GameResult::PlaceholderYeonSeung);
        println!("{:?}", result);
        println!("{:?}", best_move);
        match best_move {
            Some(x) => {
                starting_board.make_move(x);
                starting_board.print();
            },
            None => (),
        }
    }

    if all_responses {
        for legal_move in starting_board.legal_moves().iter() {
            print!("{:?} does: {:?}, ", starting_player, legal_move);
            let bt = starting_board.board_type;
            starting_board.make_move(*legal_move);
            let (result, best_move) = minimax_alpha_beta(starting_player.opponent(), &mut starting_board, GameResult::PlaceholderJunSeok, GameResult::PlaceholderYeonSeung);
            match best_move {
                Some(x) => {
                    println!("{:?} does: {:?}, {:?}", starting_player.opponent(), x, result);
                    starting_board.make_move(x);
                    starting_board.print();
                    starting_board.undo_move(x, bt);
                }
                None => (),
            }
            starting_board.undo_move(*legal_move, bt);
        }
    }

    if interactive {
        let (result, best_move) = minimax_alpha_beta(starting_player, &mut starting_board, GameResult::PlaceholderJunSeok, GameResult::PlaceholderYeonSeung);
        println!("{:?}", result);
        println!("{:?}", best_move);
        match best_move {
            Some(x) => {
                starting_board.make_move(x);
                starting_board.print();
            },
            None => (),
        }
        for legal_move in starting_board.legal_moves().iter() {
            println!("{:?}", legal_move);
        }
    }
}
