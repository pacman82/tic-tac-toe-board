mod bitboard;

use bitboard::Bitboard;
use std::{fmt, io};

/// A TacTacToe board
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct TicTacToe(Bitboard);

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe(Bitboard::new())
    }

    pub fn print_to(self, mut out: impl io::Write) -> io::Result<()> {
        let f = |i| self.0.field(CellIndex(i));

        write!(
            out,
            "-------\n\
             |{}|{}|{}|\n\
             |-----|\n\
             |{}|{}|{}|\n\
             |-----|\n\
             |{}|{}|{}|\n\
             -------",
            f(0),
            f(1),
            f(2),
            f(3),
            f(4),
            f(5),
            f(6),
            f(7),
            f(8)
        )
    }

    /// Iterator over all fields which are not occupied by a stone of either player
    pub fn open_fields(&self) -> impl Iterator<Item = CellIndex> + use<'_> {
        (0..9)
            .map(CellIndex)
            .filter(move |&i| self.0.field(i) == Cell::Empty)
    }

    pub fn state(&self) -> TicTacToeState {
        let stones = self.0.stones();
        let player = stones % 2;
        match (self.0.victory(), player) {
            (true, 0) => TicTacToeState::VictoryPlayerTwo,
            (true, 1) => TicTacToeState::VictoryPlayerOne,
            (false, 0) => TicTacToeState::TurnPlayerOne,
            _ => {
                if stones == 9 {
                    TicTacToeState::Draw
                } else {
                    TicTacToeState::TurnPlayerTwo
                }
            }
        }
    }

    /// Places a stone for the current player in the specified Cell. Panics if cell is not empty
    pub fn play_move(&mut self, &mov: &CellIndex) {
        assert!(self.0.field(mov) == Cell::Empty);
        let new_state = match self.state() {
            TicTacToeState::TurnPlayerOne => Cell::PlayerOne,
            TicTacToeState::TurnPlayerTwo => Cell::PlayerTwo,
            _ => panic!("Tic Tac Toe game is already finished."),
        };
        self.0.mark_cell(mov, new_state);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TicTacToeState {
    VictoryPlayerOne,
    VictoryPlayerTwo,
    Draw,
    TurnPlayerOne,
    TurnPlayerTwo,
}

impl TicTacToeState {
    /// `true` if the game is finished, `false` if it is still ongoing
    pub fn is_terminal(self) -> bool {
        match self {
            TicTacToeState::VictoryPlayerOne
            | TicTacToeState::VictoryPlayerTwo
            | TicTacToeState::Draw => true,
            TicTacToeState::TurnPlayerOne | Self::TurnPlayerTwo => false,
        }
    }
}

/// State of a cell in a TicTacToe Board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    /// Field is not captured by either player
    Empty,
    /// Field contains a stone from Player 1
    PlayerOne,
    /// Field contains a stone from Player 1
    PlayerTwo,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Cell::Empty => " ",
            Cell::PlayerOne => "X",
            Cell::PlayerTwo => "O",
        };
        write!(f, "{}", c)
    }
}

/// Field are enumerated 0..=8. Top left is zero. Bottom right is 8.
///
/// ```custom
/// 0 1 2
/// 3 4 5
/// 6 7 8
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CellIndex(u8);

impl CellIndex {
    /// Create a new cell index from a number between 0 and 8. Panics for values >= 9.
    ///
    /// ```custom
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    pub fn new(index: u8) -> CellIndex {
        assert!(index < 9);
        CellIndex(index)
    }

    pub fn row(self) -> u8 {
        self.0 / 3
    }

    pub fn column(self) -> u8 {
        self.0 % 3
    }
}

impl std::str::FromStr for CellIndex {
    type Err = &'static str;

    fn from_str(source: &str) -> Result<CellIndex, &'static str> {
        match source.as_bytes().first() {
            Some(v @ b'0'..=b'8') => Ok(CellIndex(v - b'0')),
            _ => Err("Only digits from 0 to 8 count as valid moves."),
        }
    }
}

impl fmt::Display for CellIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cell index: {}", self.0)
    }
}

impl From<u8> for CellIndex {
    fn from(source: u8) -> CellIndex {
        match source {
            i @ 0..=8 => CellIndex(i),
            _ => panic!("Only digits from 0 to 8 can be used as index into a tic tac toe field."),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn empty_board() {
        let board = TicTacToe::new();
        let mut buf = Vec::new();
        board.print_to(&mut buf).unwrap();

        let expected = "-------\n\
                        | | | |\n\
                        |-----|\n\
                        | | | |\n\
                        |-----|\n\
                        | | | |\n\
                        -------";

        assert_eq!(String::from_utf8(buf).unwrap(), expected);
    }

    #[test]
    fn board_with_two_stones() {
        let mut board = TicTacToe::new();
        board.0.mark_cell(CellIndex(4), Cell::PlayerOne);
        board.0.mark_cell(CellIndex(6), Cell::PlayerTwo);
        let mut buf = Vec::new();
        board.print_to(&mut buf).unwrap();

        let expected = "-------\n\
                        | | | |\n\
                        |-----|\n\
                        | |X| |\n\
                        |-----|\n\
                        |O| | |\n\
                        -------";

        assert_eq!(String::from_utf8(buf).unwrap(), expected);
    }

    #[test]
    fn victory_condition_player_two() {
        // -------
        // | | |X|
        // |-----|
        // | |X|X|
        // |-----|
        // |O|O|O|
        // -------
        let mut game = TicTacToe::new();
        game.play_move(&CellIndex::new(4));
        game.play_move(&CellIndex::new(6));
        game.play_move(&CellIndex::new(2));
        game.play_move(&CellIndex::new(8));
        game.play_move(&CellIndex::new(5));
        game.play_move(&CellIndex::new(7));

        assert_eq!(game.state(), TicTacToeState::VictoryPlayerTwo);
    }
}
