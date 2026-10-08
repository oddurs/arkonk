//! The playfield and its brick grid, in world pixels.
//!
//! The world is a fixed 960×900 frame whatever the screen; the field is the
//! walled area inside it, open at the bottom where balls drain. Bricks sit in
//! a 12 × 7 grid of cells, so collision tests only visit the cells a ball's
//! path can reach.
use crate::geom::{Rect, V2};

/// The field's left wall.
pub const LEFT: f32 = 64.0;
/// The field's right wall.
pub const RIGHT: f32 = 896.0;
/// The field's ceiling.
pub const TOP: f32 = 136.0;
/// The open bottom edge: a ball drains once it is entirely below it.
pub const BOTTOM: f32 = 814.0;
/// The walled field.
pub const FIELD: Rect = Rect {
    x: LEFT,
    y: TOP,
    w: RIGHT - LEFT,
    h: BOTTOM - TOP,
};
/// The paddle's top edge.
pub const PADDLE_Y: f32 = 770.0;
/// Every ball's radius.
pub const BALL_RADIUS: f32 = 7.0;

/// Grid columns.
pub const COLS: usize = 12;
/// Grid rows.
pub const ROWS: usize = 7;
/// Grid cells.
pub const CELLS: usize = COLS * ROWS;
/// The left edge of the first column.
pub const GRID_X: f32 = 96.0;
/// The top edge of the first row.
pub const GRID_Y: f32 = 190.0;
/// Distance between neighbouring columns.
pub const CELL_W: f32 = 64.0;
/// Distance between neighbouring rows.
pub const CELL_H: f32 = 32.0;
/// A brick's width; the rest of its cell is the gap to the next brick.
pub const BRICK_W: f32 = 58.0;
/// A brick's height.
pub const BRICK_H: f32 = 24.0;

/// One cell of the brick grid. Always in range, so indexing by it is safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell(u8);

impl Cell {
    /// The cell at row-major `index`, if there is one.
    pub const fn new(index: usize) -> Option<Self> {
        if index < CELLS {
            Some(Self(index as u8))
        } else {
            None
        }
    }
    /// Row-major position, `0..CELLS`.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// Row, counted from the top.
    pub const fn row(self) -> usize {
        self.index() / COLS
    }
    /// Column, counted from the left.
    pub const fn col(self) -> usize {
        self.index() % COLS
    }
    /// Every cell, row by row.
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        (0..CELLS as u8).map(Self)
    }
    /// The orthogonal neighbours a relay reaches, in the order it reaches
    /// them: left, right, up, down. Rows do not wrap.
    pub fn neighbors(self) -> [Option<Self>; 4] {
        let (i, row, col) = (self.index(), self.row(), self.col());
        [
            (col > 0).then(|| Self(self.0 - 1)),
            (col + 1 < COLS).then(|| Self(self.0 + 1)),
            (row > 0).then(|| Self((i - COLS) as u8)),
            (row + 1 < ROWS).then(|| Self((i + COLS) as u8)),
        ]
    }
}

/// A set of cells, one bit each.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CellSet(u128);

impl CellSet {
    /// No cells.
    pub const EMPTY: Self = Self(0);
    /// Every cell.
    pub const ALL: Self = Self((1 << CELLS) - 1);

    /// Whether `cell` is in the set.
    pub const fn contains(self, cell: Cell) -> bool {
        self.0 & (1 << cell.0) != 0
    }
    /// Adds `cell`.
    pub const fn insert(&mut self, cell: Cell) {
        self.0 |= 1 << cell.0;
    }
    /// Removes `cell`.
    pub const fn remove(&mut self, cell: Cell) {
        self.0 &= !(1 << cell.0);
    }
    /// Whether the set has no cells.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    /// The number of cells.
    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }
    /// The cells in row-major order.
    pub fn iter(self) -> impl Iterator<Item = Cell> {
        let mut bits = self.0;
        core::iter::from_fn(move || {
            if bits == 0 {
                return None;
            }
            let cell = Cell(bits.trailing_zeros() as u8);
            bits &= bits - 1;
            Some(cell)
        })
    }
}

impl FromIterator<Cell> for CellSet {
    fn from_iter<I: IntoIterator<Item = Cell>>(cells: I) -> Self {
        let mut set = Self::EMPTY;
        for cell in cells {
            set.insert(cell);
        }
        set
    }
}

/// The brick rectangle in `cell`.
pub fn cell_rect(cell: Cell) -> Rect {
    Rect {
        x: GRID_X + cell.col() as f32 * CELL_W,
        y: GRID_Y + cell.row() as f32 * CELL_H,
        w: BRICK_W,
        h: BRICK_H,
    }
}

/// The cells a ball of `radius` can touch moving in a straight line from
/// `from` to `to`, row by row. Empty when the path misses the grid.
pub fn swept_cells(from: V2, to: V2, radius: f32) -> CellRange {
    let first_col = (((from.x.min(to.x) - radius - GRID_X) / CELL_W).floor() as i32).max(0);
    let last_col =
        (((from.x.max(to.x) + radius - GRID_X) / CELL_W).floor() as i32).min(COLS as i32 - 1);
    let first_row = (((from.y.min(to.y) - radius - GRID_Y) / CELL_H).floor() as i32).max(0);
    let last_row =
        (((from.y.max(to.y) + radius - GRID_Y) / CELL_H).floor() as i32).min(ROWS as i32 - 1);
    let empty = first_col > last_col || first_row > last_row;
    CellRange {
        first_col: first_col as usize,
        last_col: last_col.max(0) as usize,
        last_row: if empty { 0 } else { last_row as usize },
        row: if empty { 1 } else { first_row as usize },
        col: first_col as usize,
    }
}

/// A rectangle of grid cells, visited row by row; see [`swept_cells`].
#[derive(Clone, Debug)]
pub struct CellRange {
    /// Leftmost column.
    first_col: usize,
    /// Rightmost column.
    last_col: usize,
    /// Bottom row.
    last_row: usize,
    /// The next cell's row; past `last_row` once the range is spent.
    row: usize,
    /// The next cell's column.
    col: usize,
}

impl Iterator for CellRange {
    type Item = Cell;
    fn next(&mut self) -> Option<Cell> {
        if self.row > self.last_row {
            return None;
        }
        let cell = Cell((self.row * COLS + self.col) as u8);
        if self.col == self.last_col {
            self.col = self.first_col;
            self.row += 1;
        } else {
            self.col += 1;
        }
        Some(cell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swept_cells_cover_the_path_and_clamp_to_the_grid() {
        let first = cell_rect(Cell::new(0).unwrap()).center();
        let cells: Vec<_> = swept_cells(first, first, BALL_RADIUS).collect();
        assert_eq!(cells, [Cell::new(0).unwrap()]);
        let across: Vec<_> = swept_cells(V2::new(0.0, first.y), V2::new(960.0, first.y), 1.0)
            .map(Cell::index)
            .collect();
        assert_eq!(across, (0..COLS).collect::<Vec<_>>());
        let corner = V2::new(GRID_X + CELL_W - 1.0, GRID_Y + CELL_H - 1.0);
        let block: Vec<_> = swept_cells(corner, corner, 2.0).map(Cell::index).collect();
        assert_eq!(block, [0, 1, COLS, COLS + 1]);
        assert_eq!(
            swept_cells(V2::new(480.0, 700.0), V2::new(500.0, 760.0), 7.0).count(),
            0
        );
        assert_eq!(
            swept_cells(V2::new(10.0, 300.0), V2::new(20.0, 300.0), 7.0).count(),
            0
        );
        assert_eq!(
            swept_cells(V2::new(950.0, 300.0), V2::new(940.0, 300.0), 7.0).count(),
            0
        );
    }

    #[test]
    fn neighbors_stop_at_the_edges() {
        let corner = Cell::new(COLS - 1).unwrap();
        let [left, right, up, down] = corner.neighbors();
        assert_eq!(left.map(Cell::index), Some(COLS - 2));
        assert_eq!((right, up), (None, None));
        assert_eq!(down.map(Cell::index), Some(2 * COLS - 1));
        assert_eq!(Cell::new(CELLS), None);
    }

    #[test]
    fn cell_sets_iterate_in_order() {
        let set: CellSet = [5, 83, 0]
            .map(|i| Cell::new(i).unwrap())
            .into_iter()
            .collect();
        assert_eq!(set.iter().map(Cell::index).collect::<Vec<_>>(), [0, 5, 83]);
        assert_eq!(set.len(), 3);
        assert_eq!(CellSet::ALL.len(), CELLS);
    }
}
