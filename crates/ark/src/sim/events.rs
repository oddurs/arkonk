//! What happened during a tick, for sound and visual feedback.

/// Flags raised during one or more ticks. Merge the ticks of a frame and
/// play each sound once.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Events {
    /// A brick took damage.
    pub brick: bool,
    /// A ball came off the paddle.
    pub paddle: bool,
    /// A ball bounced off a wall.
    pub wall: bool,
    /// The last ball drained and a life was lost.
    pub lost: bool,
    /// The sector was cleared.
    pub clear: bool,
    /// A power was picked up, or the finishing assist granted one.
    pub pickup: bool,
    /// A ball was served or released from the paddle.
    pub launch: bool,
    /// The longest chain of breaks reached, when a brick broke.
    pub combo: u32,
    /// Anchor caught a ball.
    pub caught: bool,
    /// A relay core blasted its neighbours.
    pub relay: bool,
    /// A phased ball passed through a brick.
    pub phase_hit: bool,
}

impl Events {
    /// Adds `other`'s flags to these, keeping the longer chain.
    pub fn merge(&mut self, other: Self) {
        self.brick |= other.brick;
        self.paddle |= other.paddle;
        self.wall |= other.wall;
        self.lost |= other.lost;
        self.clear |= other.clear;
        self.pickup |= other.pickup;
        self.launch |= other.launch;
        self.combo = self.combo.max(other.combo);
        self.caught |= other.caught;
        self.relay |= other.relay;
        self.phase_hit |= other.phase_hit;
    }
}
