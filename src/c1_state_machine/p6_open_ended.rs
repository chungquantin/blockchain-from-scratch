//! Now is your chance to get creative. Choose a state machine that interests you and model it here.
//! Get as fancy as you like. The only constraint is that it should be simple enough that you can
//! realistically model it in an hour or two.
//!
//! Here are some ideas:
//! - Board games:
//!   - Chess
//!   - Checkers
//!   - Tic tac toe
//! - Beaurocracies:
//!   - Beauro of Motor Vehicles - maintains driving licenses and vehicle registrations.
//!   - Public Utility Provider - Customers open accounts, consume the utility, pay their bill
//!     periodically, maybe utility prices fluctuate
//!   - Land ownership registry
//! - Tokenomics:
//!   - Token Curated Registry
//!   - Prediction Market
//!   - There's a game where there's a prize to be split among players and the prize grows over
//!     time. Any player can stop it at any point and take most of the prize for themselves.
//! - Social Systems:
//!   - Social Graph
//!   - Web of Trust
//!   - Reputation System

use anyhow::{Error, Result};
use std::collections::{HashMap, HashSet};
use std::result::Result::Ok;
use std::vec;

use super::StateMachine;

type Row = i16;
type Col = i16;
type Position = (Row, Col);

#[derive(Clone, Eq, PartialEq, Debug)]
enum Color {
    Black,
    White,
}

const BOARD_MAX_SIZE: i16 = 8;
const BOARD_MIN_SIZE: i16 = 0;

impl Color {
    fn get_other_color(self: &Self) -> Color {
        return match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };
    }

    fn dir(self: &Self) -> i16 {
        match self {
            Color::White => -1,
            Color::Black => 1,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ChessPiece {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

fn get_rook_moves((row, col): Position) -> Vec<Position> {
    let mut rook_moves = vec![];
    for i in BOARD_MIN_SIZE + 1..BOARD_MAX_SIZE {
        rook_moves.append(&mut vec![(row + i, col), (row - i, col)]);
        rook_moves.append(&mut vec![(row, col + i), (row, col - 1)]);
    }
    rook_moves
}

fn get_bishop_moves((row, col): Position) -> Vec<Position> {
    let mut bishop_moves = vec![];
    for i in BOARD_MIN_SIZE + 1..BOARD_MAX_SIZE {
        bishop_moves.append(&mut vec![
            (row + i, col + i),
            (row + i, col - i),
            (row - i, col + i),
            (row - i, col - i),
        ]);
    }
    bishop_moves
}

impl ChessPiece {
    pub fn get_color(self: &Self) -> Color {
        match self {
            ChessPiece::Pawn(color) => return color.clone(),
            ChessPiece::Bishop(color) => return color.clone(),
            ChessPiece::King(color) => return color.clone(),
            ChessPiece::Knight(color) => return color.clone(),
            ChessPiece::Queen(color) => return color.clone(),
            ChessPiece::Rook(color) => return color.clone(),
        };
    }

    pub fn get_moves(self: &Self, pos: Position) -> HashSet<Position> {
        let row = pos.0 as i16;
        let col = pos.1 as i16;
        let mut moves = HashSet::default();
        let chess_moves: Vec<Position> = match self {
            ChessPiece::Bishop(_) => get_bishop_moves(pos),
            ChessPiece::Rook(_) => get_rook_moves(pos),
            ChessPiece::King(_) => {
                vec![
                    (row + 1, col + 1),
                    (row + 1, col),
                    (row + 1, col - 1),
                    (row, col + 1),
                    (row, col - 1),
                    (row - 1, col + 1),
                    (row - 1, col),
                    (row - 1, col - 1),
                ]
            }
            ChessPiece::Knight(_) => {
                vec![
                    (row + 2, col + 1),
                    (row + 2, col - 1),
                    (row - 2, col + 1),
                    (row - 2, col - 1),
                    (row + 1, col + 2),
                    (row + 1, col - 2),
                    (row - 1, col + 2),
                    (row - 1, col - 2),
                ]
            }
            ChessPiece::Pawn(color) => {
                let d = color.dir();
                vec![(row + d, col + 1), (row - d, col - 1)]
            }
            ChessPiece::Queen(_) => {
                let rook_moves = get_rook_moves(pos);
                let bishop_moves = get_bishop_moves(pos);
                [rook_moves, bishop_moves].concat()
            }
        };
        for (row, col) in chess_moves {
            // chess moves out of the board
            if row > BOARD_MAX_SIZE
                || col > BOARD_MAX_SIZE
                || row <= BOARD_MIN_SIZE
                || col <= BOARD_MIN_SIZE
            {
                continue;
            }
            moves.insert((row, col));
        }
        return moves;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ChessGameStatus {
    Finished(Color),
    Running,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    /// Chess Board
    /// 				Column
    /// 						|
    ///   				| 1 2 3 4 5 6 7 8
    /// Row - - - - - - - - - -
    /// 				1 | R k B Q K B k R
    /// 				2 | P P P P P P P P
    /// 				3 |
    /// 				4 |
    /// 				5 |
    /// 				6 |
    /// 				7 | P P P P P P P P
    /// 				8 | R k B Q K B k R
    board: HashMap<Position, ChessPiece>,
    side_color: Color,
    status: ChessGameStatus,
    moves: u64,
}

impl Default for State {
    fn default() -> Self {
        State {
            moves: 0,
            status: ChessGameStatus::Running,
            side_color: Color::White,
            board: HashMap::from([
                ((1, 1), ChessPiece::Rook(Color::Black)),
                ((1, 2), ChessPiece::Knight(Color::Black)),
                ((1, 3), ChessPiece::Bishop(Color::Black)),
                ((1, 4), ChessPiece::Queen(Color::Black)),
                ((1, 5), ChessPiece::King(Color::Black)),
                ((1, 6), ChessPiece::Bishop(Color::Black)),
                ((1, 7), ChessPiece::Knight(Color::Black)),
                ((1, 8), ChessPiece::Rook(Color::Black)),
                ((2, 1), ChessPiece::Pawn(Color::Black)),
                ((2, 2), ChessPiece::Pawn(Color::Black)),
                ((2, 3), ChessPiece::Pawn(Color::Black)),
                ((2, 4), ChessPiece::Pawn(Color::Black)),
                ((2, 5), ChessPiece::Pawn(Color::Black)),
                ((2, 6), ChessPiece::Pawn(Color::Black)),
                ((2, 7), ChessPiece::Pawn(Color::Black)),
                ((2, 8), ChessPiece::Pawn(Color::Black)),
                ((7, 1), ChessPiece::Pawn(Color::White)),
                ((7, 2), ChessPiece::Pawn(Color::White)),
                ((7, 3), ChessPiece::Pawn(Color::White)),
                ((7, 4), ChessPiece::Pawn(Color::White)),
                ((7, 5), ChessPiece::Pawn(Color::White)),
                ((7, 6), ChessPiece::Pawn(Color::White)),
                ((7, 7), ChessPiece::Pawn(Color::White)),
                ((7, 8), ChessPiece::Pawn(Color::White)),
                ((8, 1), ChessPiece::Rook(Color::White)),
                ((8, 2), ChessPiece::Knight(Color::White)),
                ((8, 3), ChessPiece::Bishop(Color::White)),
                ((8, 4), ChessPiece::Queen(Color::White)),
                ((8, 5), ChessPiece::King(Color::White)),
                ((8, 6), ChessPiece::Bishop(Color::White)),
                ((8, 7), ChessPiece::Knight(Color::White)),
                ((8, 8), ChessPiece::Rook(Color::White)),
            ]),
        }
    }
}

pub enum Transition {
    #[allow(unused)]
    Move {
        chess_piece: ChessPiece,
        from: Position,
        to: Position,
    },
}

impl State {
    fn incre_move(self: &mut Self) {
        self.moves += 1;
    }

    fn board_move(self: &mut Self, from_pos: Position, to_pos: Position) -> Option<ChessPiece> {
        let pos_element = self.board.get(&from_pos);
        if let Some(element) = pos_element {
            let option = self.board.insert(to_pos, element.clone());
            if option.is_some() {
                return option;
            }
            self.board.remove(&from_pos);
        }
        self.incre_move();
        return None;
    }

    fn next_color(self: &mut Self, color: Color) {
        self.side_color = color.get_other_color();
    }
}

impl StateMachine for State {
    type State = State;
    type Transition = Transition;

    fn human_name() -> String {
        return String::from("Chess State Machine");
    }

    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        let mut updated_state = starting_state.clone();

        let process_state = |updated_state: &mut State| -> Result<()> {
            match t {
                Transition::Move {
                    chess_piece,
                    from,
                    to,
                } => {
                    let chess_piece_color = chess_piece.get_color();
                    let enemy_color = chess_piece_color.get_other_color();

                    if chess_piece_color != starting_state.side_color {
                        return Err(Error::msg("wrong side color"));
                    }
                    let get_chess_from_pos = starting_state.board.get(from);
                    if get_chess_from_pos.is_none() || get_chess_from_pos.unwrap() != chess_piece {
                        return Err(Error::msg("chess piece is not at `from` position"));
                    }
                    let possible_moves = chess_piece.get_moves(*from);
                    if !possible_moves.contains(to) {
                        return Err(Error::msg("invalid move"));
                    }

                    for possible_move in possible_moves {
                        if let Some(board_chess) = starting_state.board.get(&possible_move) {
                            if board_chess.get_color() == chess_piece_color {
                                // possible move lands on same side chess piece
                                return Err(Error::msg(
                                    "position is occupied by other same side chess",
                                ));
                            }
                        }
                    }
                    // this also covers a case of enemy chess piece is killed
                    if let Some(killed_chess_piece) = updated_state.board_move(*from, *to) {
                        if killed_chess_piece == ChessPiece::King(enemy_color) {
                            updated_state.status =
                                ChessGameStatus::Finished(chess_piece_color.clone());
                        }
                    }
                    updated_state.next_color(chess_piece_color);
                }
            };
            Ok(())
        };

        match process_state(&mut updated_state) {
            Ok(_) => {
                return updated_state;
            }
            Err(err) => {
                println!("{}", err.to_string());
                return starting_state.clone();
            }
        };
    }
}
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_wrong_side_color() {
        let state = State::default();
        let end = State::next_state(
            &state,
            &Transition::Move {
                chess_piece: ChessPiece::Pawn(Color::Black),
                from: (7, 1),
                to: (7, 3),
            },
        );
        let expected = State::default();
        assert_eq!(end, expected);
    }

    #[test]
    fn test_pawn_move_failed_invalid_move() {
        let state = State::default();
        let end = State::next_state(
            &state,
            &Transition::Move {
                chess_piece: ChessPiece::Pawn(Color::White),
                from: (7, 1),
                to: (8, 8),
            },
        );
        let expected = State::default();
        assert_eq!(end, expected);
    }

    #[test]
    fn test_success_move_pawn() {
        let state = State::default();
        let end = State::next_state(
            &state,
            &Transition::Move {
                chess_piece: ChessPiece::Pawn(Color::White),
                from: (7, 1),
                to: (6, 2),
            },
        );
        let mut expected = State::default();
        expected.board_move((7, 1), (6, 2));
        expected.next_color(Color::White);
        assert_eq!(end, expected);
    }

    #[test]
    fn test_invalid_bishop_move() {
        let state = State::default();
        let end = State::next_state(
            &state,
            &Transition::Move {
                chess_piece: ChessPiece::Bishop(Color::White),
                from: (8, 3),
                to: (7, 3),
            },
        );
        let expected = State::default();
        assert_eq!(end, expected);
    }

    #[test]
    fn test_success_move_bishop() {
        let state = State::default();
        let end = State::next_state(
            &state,
            &Transition::Move {
                chess_piece: ChessPiece::Bishop(Color::White),
                from: (8, 3),
                to: (7, 4),
            },
        );
        let mut expected = State::default();
        expected.board_move((8, 3), (7, 4));
        expected.next_color(Color::White);
        assert_eq!(end, expected);
    }

    #[test]
    fn test_invalid_king_move() {
        let state = State::default();
        let end = State::next_state(
            &state,
            &Transition::Move {
                chess_piece: ChessPiece::King(Color::White),
                from: (8, 5),
                to: (8, 7),
            },
        );
        let expected = State::default();
        assert_eq!(end, expected);
    }

    #[test]
    fn test_success_move_king() {
        let state = State::default();
        let end = State::next_state(
            &state,
            &Transition::Move {
                chess_piece: ChessPiece::King(Color::White),
                from: (8, 5),
                to: (8, 4),
            },
        );
        let mut expected = State::default();
        expected.board_move((8, 5), (8, 4));
        expected.next_color(Color::White);
        assert_eq!(end, expected);
    }
}

