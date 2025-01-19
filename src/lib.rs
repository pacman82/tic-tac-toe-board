use std::{fmt, io};

/// Bitboard stones
///
/// First 12 Bits encode stones of player one. Every fourth bit is zero
///  0   1   2  .
///  4   5   6  .
///  8   0  10  .
///  .   .   .  . Four bits of padding between players
///  Next 12 Bits encode stones of player two.
///  16 17 18  .
///  19 20 21  .
///  22 23 24  .
///   .  .  .  .
/// `1` represents a stone of one player. `0` is an empty field, or a stone of the other player.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
struct Bitboard(u32);

/// A TacTacToe board
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct TicTacToe(Bitboard);

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

/// Field are enumerated 0..9. Top left is zero. Bottom right is nine.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CellIndex(u8);

pub enum TicTacToeState {
    VictoryPlayerOne,
    VictoryPlayerTwo,
    Draw,
    TurnPlayerOne,
    TurnPlayerTwo,
}

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

    pub fn legal_moves(& self) -> impl Iterator<Item = CellIndex> + use<'_> {
        (0..9)
            .map(CellIndex)
            .filter(move |&i| self.0.field(i) == Cell::Empty)
    }

    pub fn state(&self) -> TicTacToeState {
        let stones = self.0.stones();
        let player = stones % 2;
        match (self.0.victory(), player) {
            (true, 0) => TicTacToeState::VictoryPlayerOne,
            (true, 1) => TicTacToeState::VictoryPlayerTwo,
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

    /// Places a stone for the current player in the specified Cell
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

impl Bitboard {
    /// An empty Tic Tac Toe board
    fn new() -> Bitboard {
        Bitboard(0)
    }

    /// Mark field at index with a stone for a player. Does not perform any checks.
    fn mark_cell(&mut self, index: CellIndex, new_state: Cell) {
        // A bitmask which is one at the cell we want to change.
        let bitmask_cell = 1 << (index.row() * (3 + 1) + index.column());
        match new_state {
            Cell::PlayerOne => self.0 |= bitmask_cell,
            Cell::PlayerTwo => self.0 |= bitmask_cell << 16,
            Cell::Empty => self.0 &= !(bitmask_cell | (bitmask_cell << 16)),
        }
    }

    fn field(self, index: CellIndex) -> Cell {
        let bitmask = 1 << (index.row() * (3 + 1) + index.column());
        if bitmask & self.0 != 0 {
            Cell::PlayerOne
        } else if (bitmask << 16) & self.0 != 0 {
            Cell::PlayerTwo
        } else {
            Cell::Empty
        }
    }

    /// True if one player has 3 stones which are allignend horizontal, diagonal or vertical
    fn victory(self) -> bool {
        let (col, row) = (1, 3 + 1);
        // horizontal or vertical or diagonal 1 or diagonal 2
        0 != (self.0 & self.0 >> col & self.0 >> (2 * col))
            | (self.0 & self.0 >> row & self.0 >> (2 * row))
            | (self.0 & self.0 >> (col + row) & self.0 >> (2 * (col + row)))
            | (self.0 & self.0 >> (row - col) & self.0 >> (2 * (row - col)))
    }

    fn stones(self) -> u8 {
        self.0.count_ones() as u8
    }
}

impl CellIndex {
    fn row(self) -> u8 {
        self.0 / 3
    }

    fn column(self) -> u8 {
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
    fn victory_condition() {
        let mut board = Bitboard::new();
        assert!(!board.victory());
        board.mark_cell(CellIndex(0), Cell::PlayerTwo);
        board.mark_cell(CellIndex(4), Cell::PlayerTwo);
        board.mark_cell(CellIndex(8), Cell::PlayerTwo);
        assert!(board.victory());
    }
}
