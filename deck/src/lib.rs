use std::{slice::Iter, fmt::Display};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds
}

impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        use self::Suit::*;
        static _SUITS: [Suit; 4] = [
            Spades,
            Clubs,
            Hearts,
            Diamonds
        ];
        _SUITS.iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Value {
    pub fn iter() -> Iter<'static, Value> {
        use self::Value::*;
        static _VALUES: [Value; 13] = [
            Ace,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Ten,
            Jack,
            Queen,
            King
        ];
        _VALUES.iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self {
            suit,
            value
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.value, self.suit)
    }
}

#[derive(Debug, PartialEq)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        
        for suit in Suit::iter() {
            for value in Value::iter() {
                cards.push(Card::new(*suit, *value));
            }
        }

        Self {
            cards
        }
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        for suit in Suit::iter() {
            for value in Value::iter() {
                self.cards.push(Card::new(*suit, *value));
            }
        }
    }

    pub fn shuffle(&mut self) {
        /*
            // Fisher-Yates shuffle
            To shuffle an array a of n elements (indices 0..n-1):
            for i from n - 1 downto 1 do
                j = random integer with 0 <= j <= i
                exchange a[j] and a[i]
        */
        for i in (1..self.cards.len()-1).rev() {
            let j = rand::thread_rng().gen_range(0..(self.cards.len()-1));
            self.cards.swap(j, i);
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_multiple(&mut self, n: usize) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..n {
            match self.draw() {
                Some(card) => cards.push(card),
                None => return cards
            }
        }
        cards
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_deck() {
        let deck = Deck::new();
        /* for card in &deck.cards {
            println!("Card {{\n\tsuit: Suit::{:?},\n\tvalue: Value::{:?}\n}},", card.suit, card.value);
        } */
        assert_eq!(Deck {
            cards: vec![
                Card {
                    suit: Suit::Spades,
                    value: Value::Ace
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Two
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Three
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Four
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Five
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Six
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Seven
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Eight
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Nine
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Ten
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Jack
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::Queen
                },
                Card {
                    suit: Suit::Spades,
                    value: Value::King
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Ace
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Two
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Three
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Four
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Five
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Six
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Seven
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Eight
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Nine
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Ten
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Jack
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::Queen
                },
                Card {
                    suit: Suit::Clubs,
                    value: Value::King
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Ace
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Two
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Three
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Four
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Five
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Six
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Seven
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Eight
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Nine
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Ten
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Jack
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::Queen
                },
                Card {
                    suit: Suit::Hearts,
                    value: Value::King
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Ace
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Two
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Three
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Four
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Five
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Six
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Seven
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Eight
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Nine
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Ten
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Jack
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::Queen
                },
                Card {
                    suit: Suit::Diamonds,
                    value: Value::King
                },
            ]
        }, deck);
    }

    #[test]
    fn shuffle_deck() {
        let deck = Deck::new();
        let mut deck_to_shuffle = Deck::new();
        deck_to_shuffle.shuffle();
        assert_ne!(deck, deck_to_shuffle)
    }

    #[test]
    fn draw_card() {
        let mut deck = Deck::new();
        let len = deck.cards.len();
        let card = deck.draw().unwrap();
        assert_eq!(deck.cards.len(), len-1);
        assert!(!deck.cards.contains(&card));
    }
}
