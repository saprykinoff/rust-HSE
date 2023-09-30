#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub trait Player {
    fn cooperate(&self) -> bool;
    fn other_s_turn(&mut self, turn: bool);
}

pub struct Game {
    // TODO: your code goes here.
    left: Box<dyn Player>,
    right: Box<dyn Player>,
    left_score: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Player>, right: Box<dyn Player>) -> Self {
        Game {
            left,
            right,
            left_score: 0,
            right_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let l = self.left.cooperate();
        let r = self.right.cooperate();
        self.left.other_s_turn(r);
        self.right.other_s_turn(l);

        if l && r {
            self.left_score += 2;
            self.right_score += 2;
            RoundOutcome::BothCooperated
        } else if l && !r {
            self.right_score += 3;
            self.left_score -= 1;
            RoundOutcome::RightCheated
        } else if !l && r {
            self.left_score += 3;
            self.right_score -= 1;
            RoundOutcome::LeftCheated
        } else {
            RoundOutcome::BothCheated
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {
    // TODO: your code goes here.
}
impl Player for CheatingAgent {
    fn cooperate(&self) -> bool {
        false
    }

    fn other_s_turn(&mut self, _turn: bool) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Player for CooperatingAgent {
    fn cooperate(&self) -> bool {
        true
    }

    fn other_s_turn(&mut self, _turn: bool) {}
}
////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {
    trust: bool,
}
impl Default for GrudgerAgent {
    fn default() -> Self {
        GrudgerAgent { trust: true }
    }
}
impl Player for GrudgerAgent {
    fn cooperate(&self) -> bool {
        self.trust
    }

    fn other_s_turn(&mut self, turn: bool) {
        if !turn {
            self.trust = false;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    trust: bool,
}
impl Default for CopycatAgent {
    fn default() -> Self {
        CopycatAgent { trust: true }
    }
}
impl Player for CopycatAgent {
    fn cooperate(&self) -> bool {
        self.trust
    }

    fn other_s_turn(&mut self, turn: bool) {
        self.trust = turn;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    trust: bool,
    num: i32,
    other_cheated: bool,
}

impl Default for DetectiveAgent {
    fn default() -> Self {
        DetectiveAgent {
            trust: true,
            num: 0,
            other_cheated: false,
        }
    }
}

impl Player for DetectiveAgent {
    fn cooperate(&self) -> bool {
        if self.num < 4 {
            self.num != 1
        } else if self.other_cheated {
            self.trust
        } else {
            false
        }
    }

    fn other_s_turn(&mut self, turn: bool) {
        if self.num >= 3 {
            self.trust = turn;
        } else if !turn {
            self.other_cheated = true;
        }

        self.num += 1;
    }
}
