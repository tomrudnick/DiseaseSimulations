#[derive(PartialEq, Eq)]
pub enum State {
    Infected,
    Healthy,
}

#[derive(PartialEq, Eq)]
pub enum InfectProgress {
    Left,
    TwoLeft,
    Right,
    TwoRight,
    Heal,
}
