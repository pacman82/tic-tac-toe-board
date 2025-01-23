use crate::{Cell, CellIndex};

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
pub struct Bitboard(u32);

impl Bitboard {
    /// An empty Tic Tac Toe board
    pub fn new() -> Bitboard {
        Bitboard(0)
    }

    /// Mark field at index with a stone for a player. Does not perform any checks.
    pub fn mark_cell(&mut self, index: CellIndex, new_state: Cell) {
        // A bitmask which is one at the cell we want to change.
        let bitmask_cell = 1 << (index.row() * (3 + 1) + index.column());
        match new_state {
            Cell::PlayerOne => self.0 |= bitmask_cell,
            Cell::PlayerTwo => self.0 |= bitmask_cell << 16,
            Cell::Empty => self.0 &= !(bitmask_cell | (bitmask_cell << 16)),
        }
    }

    pub fn field(self, index: CellIndex) -> Cell {
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
    pub fn victory(self) -> bool {
        let (col, row) = (1, 3 + 1);
        // horizontal or vertical or diagonal 1 or diagonal 2
        0 != (self.0 & self.0 >> col & self.0 >> (2 * col))
            | (self.0 & self.0 >> row & self.0 >> (2 * row))
            | (self.0 & self.0 >> (col + row) & self.0 >> (2 * (col + row)))
            | (self.0 & self.0 >> (row - col) & self.0 >> (2 * (row - col)))
    }

    pub fn stones(self) -> u8 {
        self.0.count_ones() as u8
    }
}

#[cfg(test)]
mod test {

    use super::*;

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