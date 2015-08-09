extern crate monorail;

use monorail::player::Player;
use std::collections::BTreeSet;
use std::env;
use std::io;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn move_in(&self, dir: Direction, delta: usize) -> Option<Coordinate> {
        match dir {
            Direction::Up => if self.row >= delta { Some(Coordinate{row: self.row - delta, col: self.col}) } else { None },
            Direction::Down => if self.row + delta < NUM_ROWS { Some(Coordinate{row: self.row + delta, col: self.col}) } else { None },
            Direction::Left => if self.col >= delta { Some(Coordinate{row: self.row, col: self.col - delta}) } else { None },
            Direction::Right => if self.col + delta < NUM_COLS { Some(Coordinate{row: self.row, col: self.col + delta}) } else { None },
        }
    }
    fn induces_board_type(&self) -> bool {
        // The lower left corner of the board.
        self.col < 2 && self.row >= 1
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
    new_board_type: Option<BoardType>,
}
impl Move {
    fn extensions(&self) -> Vec<Coordinate> {
        match self.move_type {
            MoveType::Single => vec![],
            MoveType::OneUp => vec![self.coord.move_in(Direction::Up, 1).unwrap()],
            MoveType::OneDown => vec![self.coord.move_in(Direction::Down, 1).unwrap()],
            MoveType::OneLeft => vec![self.coord.move_in(Direction::Left, 1).unwrap()],
            MoveType::OneRight => vec![self.coord.move_in(Direction::Right, 1).unwrap()],
            MoveType::TwoUp => vec![self.coord.move_in(Direction::Up, 1).unwrap(), self.coord.move_in(Direction::Up, 2).unwrap()],
            MoveType::TwoDown => vec![self.coord.move_in(Direction::Down, 1).unwrap(), self.coord.move_in(Direction::Down, 2).unwrap()],
            MoveType::TwoLeft => vec![self.coord.move_in(Direction::Left, 1).unwrap(), self.coord.move_in(Direction::Left, 2).unwrap()],
            MoveType::TwoRight => vec![self.coord.move_in(Direction::Right, 1).unwrap(), self.coord.move_in(Direction::Right, 2).unwrap()],
            MoveType::UpAndDown => vec![self.coord.move_in(Direction::Up, 1).unwrap(), self.coord.move_in(Direction::Down, 1).unwrap()],
            MoveType::LeftAndRight => vec![self.coord.move_in(Direction::Left, 1).unwrap(), self.coord.move_in(Direction::Right, 1).unwrap()],
        }
    }
}

const NUM_COLS: usize = 5;
const NUM_ROWS: usize = 4;

// Hacks for the three states of the lower-left of the board in JunSeok vs YeonSeung game
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum BoardType {
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

const POSSIBLE_BOARD_TYPES: [BoardType; 5] = [
    BoardType::Left,
    BoardType::LeftOrMiddle,
    BoardType::Middle,
    BoardType::RightOrMiddle,
    BoardType::Right,
];

type BoardArray = [[bool; NUM_COLS]; NUM_ROWS];
struct Board {
    board: BoardArray,
    board_type: Option<BoardType>,
}

impl Board {
    fn make_move(&mut self, m: Move) {
        if let Some(bt) = m.new_board_type {
            if !bt.applies_to(self.board_type) {
                panic!("Board type is {:?}, not compatible with {:?}", self.board_type, bt);
            }
            self.board_type = m.new_board_type
        }
        self.set_squares(m, true)
    }

    fn undo_move(&mut self, m: Move, bt: Option<BoardType>) {
        self.board_type = bt;
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
                    if let Some(dest) = coord.move_in(*dir, 1) { self.occupied(dest) } else { false }
                });
                if have_neighbor {
                    results.push(coord);
                }
            }
        }
        results
    }

    fn board_type_final(&self) -> bool {
        if let Some(x) = self.board_type { x.is_final() } else { false }
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut results = Vec::new();
        for frontier_space in self.frontier().iter() {
            for move_type in POSSIBLE_MOVE_TYPES.iter() {
                let mov = Move{coord: *frontier_space, move_type: *move_type, new_board_type: None};
                if !self.move_in_bounds(mov) {
                    continue;
                }
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
                if !other_space_taken {
                    if induces_board_type && !self.board_type_final() {
                        let mut ok_board_types = BTreeSet::new();
                        for board_type in POSSIBLE_BOARD_TYPES.iter() {
                            if !board_type.applies_to(self.board_type) {
                                continue;
                            }
                            if !board_type.induced_by(*frontier_space) {
                                continue;
                            }
                            if mov.extensions().iter().all(|coord| board_type.induced_by(*coord)) {
                                ok_board_types.insert(*board_type);
                            }
                        }

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
                            results.push(Move{coord: mov.coord, move_type: mov.move_type, new_board_type: Some(*board_type)});
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
        // Print header row
        print!("   ");
        for i in 0..NUM_COLS {
            print!("{: >5} ", i);
        }
        println!("");

        for (i, row) in self.board.iter().enumerate() {
            print!("{: >2} ", i);
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
    PlaceholderAlpha,
    JunSeokWin,
    YeonSeungWin,
    PlaceholderBeta,
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

fn print_all_responses(player: Player, starting_board: &mut Board) {
    for legal_move in starting_board.legal_moves().iter() {
        print!("If {:?} does: {:?}, ", player, legal_move);
        let bt = starting_board.board_type;
        starting_board.make_move(*legal_move);
        let (result, best_move) = minimax_alpha_beta(player.opponent(), starting_board, GameResult::PlaceholderAlpha, GameResult::PlaceholderBeta);
        match best_move {
            Some(x) => {
                println!("{:?} does: {:?}, {:?}", player.opponent(), x, result);
                starting_board.make_move(x);
                starting_board.print();
                starting_board.undo_move(x, bt);
            }
            None => (),
        }
        starting_board.undo_move(*legal_move, bt);
    }
}

fn print_best_move(player: Player, starting_board: &mut Board) {
    let (result, best_move) = minimax_alpha_beta(player, starting_board, GameResult::PlaceholderAlpha, GameResult::PlaceholderBeta);
    println!("{:?}", result);
    println!("{:?}", best_move);
    match best_move {
        Some(x) => {
            let bt = starting_board.board_type;
            starting_board.make_move(x);
            starting_board.print();
            starting_board.undo_move(x, bt);
        },
        None => (),
    }
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

    let mut all_responses = false;
    let mut best_move = false;
    let mut legal_moves = false;

    for argument in env::args() {
        if argument == "-b" {
            best_move = true;
        }
        if argument == "-a" {
            all_responses = true;
        }
        if argument == "-l" {
            legal_moves = true;
        }
    }

    let interactive = !all_responses && !best_move && !legal_moves;

    if legal_moves {
        for legal_move in starting_board.legal_moves().iter() {
            println!("{:?}", legal_move);
        }
    }

    if best_move {
        print_best_move(starting_player, &mut starting_board);
    }

    if all_responses {
        print_all_responses(starting_player, &mut starting_board);
    }

    if interactive {
        let mut player = starting_player;
        let mut turn_counter = 1;
        loop {
            println!("=================== Turn {} ===================", turn_counter);
            let moves = starting_board.legal_moves();
            if moves.is_empty() {
                println!("No moves left, {:?} wins", player.opponent());
                break;
            }
            starting_board.print();
            for (i, legal_move) in moves.iter().enumerate() {
                println!("{} {:?}", i, legal_move);
            }
            println!("It's {:?}'s turn. What move?", player);
            let mut input_move = String::new();
            io::stdin().read_line(&mut input_move).ok().expect("Failed to read line");
            let input_move = input_move.trim();
            if input_move == "analyze" || input_move == "a" {
                print_all_responses(player, &mut starting_board);
            } else if input_move == "best" || input_move == "b" {
                print_best_move(player, &mut starting_board);
            } else {
                let input_move: usize = match input_move.trim().parse() {
                    Ok(num) => num,
                    Err(_) => { println!("Not a number."); continue },
                };
                match moves.get(input_move) {
                    Some(legal_move) => {
                        starting_board.make_move(*legal_move);
                        player = player.opponent();
                        turn_counter += 1;
                    },
                    None => println!("Move not found.")
                }
            }
        }
    }
}
