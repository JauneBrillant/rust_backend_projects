#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: usize,
}
