use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use crate::card::{Card, Suit};

pub enum PokerHand {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl fmt::Display for PokerHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hand_str = match self {
            PokerHand::RoyalFlush => "ロイヤルストレートフラッシュ",
            PokerHand::StraightFlush => "ストレートフラッシュ",
            PokerHand::FourOfAKind => "フォーカード",
            PokerHand::FullHouse => "フルハウス",
            PokerHand::Flush => "フラッシュ",
            PokerHand::Straight => "ストレート",
            PokerHand::ThreeOfAKind => "スリーカード",
            PokerHand::TwoPair => "ツーペア",
            PokerHand::OnePair => "ワンペア",
            PokerHand::HighCard => "ハイカード",
        };
        write!(f, "{}", hand_str)
    }
}

pub fn evaluate_hand(hand: Vec<Card>) -> PokerHand {
    if royal_flush(&hand) {
        PokerHand::RoyalFlush
    } else if straight_flush(&hand) {
        PokerHand::StraightFlush
    } else if four_of_a_kind(&hand) {
        PokerHand::FourOfAKind
    } else if full_house(&hand) {
        PokerHand::FullHouse
    } else if flush(&hand) {
        PokerHand::Flush
    } else if straight(&hand) {
        PokerHand::Straight
    } else if three_of_a_kind(&hand) {
        PokerHand::ThreeOfAKind
    } else if two_pair(&hand) {
        PokerHand::TwoPair
    } else if one_pair(&hand) {
        PokerHand::OnePair
    } else {
        PokerHand::HighCard
    }
}

fn royal_flush(hand: &[Card]) -> bool {
    let required_ranks: HashSet<usize> = [10, 11, 12, 13, 1].into_iter().collect();
    let suits: HashSet<Suit> = hand.iter().map(|c| c.suit).collect();
    let ranks: HashSet<usize> = hand.iter().map(|c| c.rank).collect();

    suits.len() == 1 && ranks == required_ranks
}

fn straight_flush(hand: &[Card]) -> bool {
    let suits: HashSet<Suit> = hand.iter().map(|e| e.suit).collect();
    if suits.len() > 1 {
        return false;
    }

    let mut ranks: Vec<_> = hand.iter().map(|e| e.rank).collect();
    ranks.sort();

    if ranks == vec![1, 10, 11, 12, 13] || ranks.windows(2).all(|w| w[0] + 1 == w[1]) {
        return true;
    }

    false
}

fn four_of_a_kind(hand: &[Card]) -> bool {
    let mut rank_count = HashMap::new();

    for card in hand {
        *rank_count.entry(card.rank).or_insert(0) += 1;
    }

    rank_count.values().any(|&cnt| cnt == 4)
}

fn full_house(hand: &[Card]) -> bool {
    let mut rank_count = HashMap::new();

    for card in hand {
        *rank_count.entry(card.rank).or_insert(0) += 1;
    }

    let mut values: Vec<i32> = rank_count.values().cloned().collect();
    values.sort_by(|a, b| b.cmp(a));

    values[0] == 3 && values[1] == 2
}

fn flush(hand: &[Card]) -> bool {
    let suits: HashSet<Suit> = hand.iter().map(|c| c.suit).collect();

    suits.len() == 1
}

fn straight(hand: &[Card]) -> bool {
    let mut ranks: Vec<usize> = hand.iter().map(|c| c.rank).collect();
    ranks.sort();

    ranks == vec![1, 10, 11, 12, 13] || ranks.windows(2).all(|w| w[0] + 1 == w[1])
}

fn three_of_a_kind(hand: &[Card]) -> bool {
    let mut rank_count = HashMap::new();
    for card in hand {
        *rank_count.entry(card.rank).or_insert(0) += 1;
    }

    rank_count.values().any(|&cnt| cnt == 3)
}

fn two_pair(hand: &[Card]) -> bool {
    let mut rank_count = HashMap::new();
    for card in hand {
        *rank_count.entry(card.rank).or_insert(0) += 1;
    }

    let mut values: Vec<i32> = rank_count.values().cloned().collect();
    values.sort_by(|a, b| b.cmp(a));

    values[0] == 2 && values[1] == 2
}

fn one_pair(hand: &[Card]) -> bool {
    let mut rank_count = HashMap::new();
    for card in hand {
        *rank_count.entry(card.rank).or_insert(0) += 1;
    }

    rank_count.values().any(|&n| n == 2)
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_royal_flush() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 10,
            },
            Card {
                suit: Suit::Heart,
                rank: 11,
            },
            Card {
                suit: Suit::Heart,
                rank: 12,
            },
            Card {
                suit: Suit::Heart,
                rank: 13,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
        ];
        assert!(royal_flush(&hand));
    }

    #[test]
    fn test_straight_flush_1() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 2,
            },
            Card {
                suit: Suit::Heart,
                rank: 3,
            },
            Card {
                suit: Suit::Heart,
                rank: 4,
            },
            Card {
                suit: Suit::Heart,
                rank: 5,
            },
        ];
        assert!(straight_flush(&hand));
    }

    #[test]
    fn test_straight_flush_2() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 10,
            },
            Card {
                suit: Suit::Heart,
                rank: 11,
            },
            Card {
                suit: Suit::Heart,
                rank: 12,
            },
            Card {
                suit: Suit::Heart,
                rank: 13,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
        ];
        assert!(straight_flush(&hand));
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Diamond,
                rank: 1,
            },
            Card {
                suit: Suit::Spade,
                rank: 1,
            },
            Card {
                suit: Suit::Club,
                rank: 13,
            },
        ];
        assert!(four_of_a_kind(&hand));
    }

    #[test]
    fn test_full_house() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Diamond,
                rank: 1,
            },
            Card {
                suit: Suit::Spade,
                rank: 2,
            },
            Card {
                suit: Suit::Club,
                rank: 2,
            },
        ];
        assert!(full_house(&hand));
    }

    #[test]
    fn test_flush() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 3,
            },
            Card {
                suit: Suit::Heart,
                rank: 6,
            },
            Card {
                suit: Suit::Heart,
                rank: 9,
            },
            Card {
                suit: Suit::Heart,
                rank: 12,
            },
        ];
        assert!(flush(&hand));
    }

    #[test]
    fn test_straight_1() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 2,
            },
            Card {
                suit: Suit::Heart,
                rank: 3,
            },
            Card {
                suit: Suit::Diamond,
                rank: 4,
            },
            Card {
                suit: Suit::Diamond,
                rank: 5,
            },
        ];
        assert!(straight(&hand));
    }

    #[test]
    fn test_straight_2() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 11,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 12,
            },
            Card {
                suit: Suit::Diamond,
                rank: 13,
            },
            Card {
                suit: Suit::Diamond,
                rank: 10,
            },
        ];
        assert!(straight(&hand));
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Diamond,
                rank: 1,
            },
            Card {
                suit: Suit::Spade,
                rank: 13,
            },
            Card {
                suit: Suit::Club,
                rank: 13,
            },
        ];
        assert!(three_of_a_kind(&hand));
    }

    #[test]
    fn test_two_pair() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Diamond,
                rank: 2,
            },
            Card {
                suit: Suit::Spade,
                rank: 2,
            },
            Card {
                suit: Suit::Club,
                rank: 3,
            },
        ];
        assert!(two_pair(&hand));
    }

    #[test]
    fn test_one_pair() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Diamond,
                rank: 2,
            },
            Card {
                suit: Suit::Spade,
                rank: 3,
            },
            Card {
                suit: Suit::Club,
                rank: 4,
            },
        ];
        assert!(one_pair(&hand));
    }

    #[test]
    fn test_high_card() {
        let hand = vec![
            Card {
                suit: Suit::Heart,
                rank: 1,
            },
            Card {
                suit: Suit::Heart,
                rank: 3,
            },
            Card {
                suit: Suit::Diamond,
                rank: 5,
            },
            Card {
                suit: Suit::Spade,
                rank: 7,
            },
            Card {
                suit: Suit::Club,
                rank: 9,
            },
        ];

        assert!(!royal_flush(&hand));
        assert!(!straight_flush(&hand));
        assert!(!four_of_a_kind(&hand));
        assert!(!full_house(&hand));
        assert!(!flush(&hand));
        assert!(!straight(&hand));
        assert!(!three_of_a_kind(&hand));
        assert!(!two_pair(&hand));
        assert!(!one_pair(&hand));
    }
}
