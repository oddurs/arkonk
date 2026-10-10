//! The bricks: hit points, relay cores, gates and the blasts they have
//! pending.
use crate::{
    field::{CELLS, Cell, CellSet},
    sectors::Layout,
    tuning::RELAY_TICKS,
};

/// What one point of damage did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Damage {
    /// The cell was already empty.
    Missed,
    /// An armoured brick lost a hit point and survives.
    Chipped,
    /// The brick broke. A core starts its relay countdown.
    Broken,
}

/// The brick grid. Play changes hit points only by damaging bricks, so the
/// count of remaining bricks can never disagree with the grid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board {
    /// Hit points per cell; zero is empty.
    hp: [u8; CELLS],
    /// Relay cores, intact or destroyed.
    cores: CellSet,
    /// Gates, which are ghosts while `ghosts` is set.
    gates: CellSet,
    /// Whether the gates are ghosts now: no collision, no damage.
    ghosts: bool,
    /// Ticks until a destroyed core's blast; zero when none is pending.
    relays: [u8; CELLS],
    /// Bricks with hit points left.
    remaining: usize,
    /// Bricks the board started with.
    initial: usize,
}

impl Board {
    /// The board a sector starts with.
    pub(crate) fn new(layout: &Layout) -> Self {
        let mut board = Self {
            hp: [0; CELLS],
            cores: CellSet::EMPTY,
            gates: CellSet::EMPTY,
            ghosts: false,
            relays: [0; CELLS],
            remaining: 0,
            initial: 0,
        };
        board.reset(layout.hp, layout.cores);
        board.gates = layout.gates;
        board
    }

    /// Hit points of the brick in `cell`; zero when empty.
    pub fn hp(&self, cell: Cell) -> u8 {
        self.hp[cell.index()]
    }
    /// Whether `cell` is a relay core, intact or destroyed.
    pub fn is_core(&self, cell: Cell) -> bool {
        self.cores.contains(cell)
    }
    /// Whether `cell` is a gate, intact or broken.
    pub fn is_gate(&self, cell: Cell) -> bool {
        self.gates.contains(cell)
    }
    /// Whether the gates are ghosts now, letting balls and blasts through.
    pub fn gates_are_ghosts(&self) -> bool {
        self.ghosts
    }
    /// Whether a ball meets a brick in `cell`: it has hit points and is not
    /// a ghost gate.
    pub fn is_solid(&self, cell: Cell) -> bool {
        self.hp[cell.index()] > 0 && !(self.ghosts && self.gates.contains(cell))
    }
    /// Ticks until the destroyed core in `cell` blasts its neighbours; zero
    /// when no blast is pending there.
    pub fn relay_countdown(&self, cell: Cell) -> u8 {
        self.relays[cell.index()]
    }
    /// Whether any blast is still pending. A sector cannot end before its
    /// chains finish.
    pub fn relays_pending(&self) -> bool {
        self.relays.iter().any(|&ticks| ticks > 0)
    }
    /// Bricks with hit points left.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
    /// Bricks the board started with.
    pub fn initial(&self) -> usize {
        self.initial
    }

    /// Replaces every brick, as if the sector had started with `hp` and
    /// `cores` and no gates. Pending blasts are cancelled.
    pub fn reset(&mut self, hp: [u8; CELLS], cores: CellSet) {
        self.hp = hp;
        self.cores = cores;
        self.gates = CellSet::EMPTY;
        self.ghosts = false;
        self.relays = [0; CELLS];
        self.remaining = hp.iter().filter(|&&hp| hp > 0).count();
        self.initial = self.remaining;
    }

    /// Sets one cell's hit points and core flag, keeping the remaining count.
    pub fn set(&mut self, cell: Cell, hp: u8, core: bool) {
        let before = self.hp[cell.index()] > 0;
        self.hp[cell.index()] = hp;
        if core {
            self.cores.insert(cell);
        } else {
            self.cores.remove(cell);
        }
        self.remaining = self.remaining + usize::from(hp > 0) - usize::from(before);
    }

    /// Makes `cell` a gate or an ordinary brick, keeping its hit points.
    pub fn set_gate(&mut self, cell: Cell, gate: bool) {
        if gate {
            self.gates.insert(cell);
        } else {
            self.gates.remove(cell);
        }
    }

    /// Turns every gate solid or ghost.
    pub(crate) fn set_ghosts(&mut self, ghosts: bool) {
        self.ghosts = ghosts;
    }

    /// Takes one hit point from the brick in `cell`. A ghost gate takes
    /// none.
    pub(crate) fn damage(&mut self, cell: Cell) -> Damage {
        if !self.is_solid(cell) {
            return Damage::Missed;
        }
        let hp = &mut self.hp[cell.index()];
        *hp -= 1;
        if *hp > 0 {
            return Damage::Chipped;
        }
        self.remaining -= 1;
        if self.cores.contains(cell) {
            self.relays[cell.index()] = RELAY_TICKS;
        }
        Damage::Broken
    }

    /// Counts pending blasts down one tick and returns the cells whose blast
    /// is due now. Taking them all before any goes off means a core that
    /// one of these blasts destroys waits its own full delay, whatever order
    /// the caller visits them in.
    pub(crate) fn tick_relays(&mut self) -> CellSet {
        let mut due = CellSet::EMPTY;
        for cell in Cell::all() {
            let ticks = &mut self.relays[cell.index()];
            if *ticks > 0 {
                *ticks -= 1;
                if *ticks == 0 {
                    due.insert(cell);
                }
            }
        }
        due
    }
}
