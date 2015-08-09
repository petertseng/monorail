extern crate monorail;
extern crate term;

use monorail::action::Move;
use monorail::board::Board;
use monorail::player::Player;
use std::env;
use std::io;

#[derive(Copy, Clone, Debug)]
enum GameResult {
    JunSeokWin,
    YeonSeungWin,
}

impl GameResult {
    fn win_for(&self, p: Player) -> bool {
        match (*self, p) {
            (GameResult::JunSeokWin, Player::JunSeok) => true,
            (GameResult::JunSeokWin, _) => false,
            (GameResult::YeonSeungWin, Player::YeonSeung) => true,
            (GameResult::YeonSeungWin, _) => false,
        }
    }
}

fn game_result(player: Player, board: &mut Board) -> (GameResult, Option<Move>) {
    let moves = board.legal_moves();
    // There are no more moves, which means my opponent completed the railroad.
    // So I lose.
    if moves.is_empty() {
        return match player {
            Player::YeonSeung => (GameResult::JunSeokWin, None),
            Player::JunSeok => (GameResult::YeonSeungWin, None),
        }
    }

    for possible_move in moves.iter() {
        board.make_move(*possible_move);
        let (reply, _) = game_result(player.opponent(), board);
        board.undo_move();

        // If I have any move that forces a win, I use that move to win.
        // We can return early from the search.
        match (player, reply) {
            (Player::YeonSeung, GameResult::YeonSeungWin) => return (reply, Some(*possible_move)),
            (Player::JunSeok, GameResult::JunSeokWin) => return (reply, Some(*possible_move)),
            _ => (),
        }
    }

    // I have no move that forces a win, therefore I must have lost.
    match player {
        Player::YeonSeung => (GameResult::JunSeokWin, None),
        Player::JunSeok => (GameResult::YeonSeungWin, None),
    }
}

fn print_result(result: GameResult, color: term::color::Color, colorize: bool) {
    if colorize {
        let mut t = term::stdout().unwrap();
        t.fg(color).unwrap();
        t.attr(term::Attr::Bold).unwrap();
        writeln!(t, "{:?}", result).unwrap();
        t.reset().unwrap();
    } else {
        println!("{:?}", result);
    }
}

fn print_all_responses(player: Player, starting_board: &mut Board, colorize: bool) {
    for legal_move in starting_board.legal_moves().iter() {
        print!("If {:?} does: {}, ", player, legal_move);
        starting_board.make_move(*legal_move);
        let (result, best_move) = game_result(player.opponent(), starting_board);
        if result.win_for(player) {
            print_result(result, term::color::BLUE, colorize);
            println!("{}", starting_board);
        } else if let Some(opponent_move) = best_move {
            print!("{:?} does: {}, ", player.opponent(), opponent_move);
            print_result(result, term::color::RED, colorize);
            starting_board.make_move(opponent_move);
            println!("{}", starting_board);
            starting_board.undo_move();
        } else {
            panic!("no move?");
        }
        starting_board.undo_move();
    }
}

fn print_best_move(player: Player, starting_board: &mut Board) {
    let (result, best_move) = game_result(player, starting_board);
    println!("{:?}", result);
    match best_move {
        Some(x) => {
            println!("{}", x);
            starting_board.make_move(x);
            println!("{}", starting_board);
            starting_board.undo_move();
        },
        None => println!("No move"),
    }
}

fn main() {
    let mut starting_board = Board::new(
        [
            [false,  true,  true,  true, false],
            [false, false, false,  true, false],
            [false, false, false,  true, false],
            [false, false, false, false, false],
        ],
        None,
    );
    let starting_player = Player::YeonSeung;

    let mut all_responses = false;
    let mut best_move = false;
    let mut legal_moves = false;
    let mut colorize = false;

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
        if argument == "-c" {
            colorize = true;
        }
    }

    let interactive = !all_responses && !best_move && !legal_moves;

    if legal_moves {
        for legal_move in starting_board.legal_moves().iter() {
            println!("{}", legal_move);
        }
    }

    if best_move {
        print_best_move(starting_player, &mut starting_board);
    }

    if all_responses {
        print_all_responses(starting_player, &mut starting_board, colorize);
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
            println!("{}", starting_board);
            for (i, legal_move) in moves.iter().enumerate() {
                println!("{} {}", i, legal_move);
            }
            println!("It's {:?}'s turn. What move?", player);
            let mut input_move = String::new();
            io::stdin().read_line(&mut input_move).ok().expect("Failed to read line");
            let input_move = input_move.trim();
            if input_move == "analyze" || input_move == "a" {
                print_all_responses(player, &mut starting_board, colorize);
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
