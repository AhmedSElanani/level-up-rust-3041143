#![allow(dead_code)]

enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        return Hand { cards: vec![] };
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut sum = 0;

        for card in self.cards.iter() {
            sum += match card {
                Card::Ace => {
                    if sum > 10 {
                        1
                    } else {
                        11
                    }
                }
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Jack | Card::Queen | Card::King => 10,
            };
        }

        return sum;
    }

    fn is_loosing_hand(&self) -> bool {
        return self.value() > 21;
    }
}

fn main() {
    let mut hand = Hand::new();

    hand.add(Card::King);
    hand.add(Card::Ace);
}

// Tests from the course
#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}
