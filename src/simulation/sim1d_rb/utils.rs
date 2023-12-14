
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum State {
    Infected,
    Healthy,
}

#[derive(PartialEq, Eq)]
pub enum InfectProgress {
    Left,
    Right,
    Heal,
}
