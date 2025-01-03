use simple_poker::card::{Card, Suit};
use simple_poker::evaluation::evaluate_hand;

use rand::seq::SliceRandom;

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    let mut hand = Vec::new();
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("--- Hand ---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    println!("\n入れ替えたいカードの番号を入力してください (例: 1 2 3)\n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("\n{}", evaluate_hand(hand));
}
