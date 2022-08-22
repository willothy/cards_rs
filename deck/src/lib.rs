use std::{slice::Iter, fmt::Display};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
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
enum Value {
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
struct Card {
    suit: Suit,
    value: Value
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
        write!(f, "{:?} of {:?}\n", self.value, self.suit)
    }
}

#[derive(Debug, PartialEq)]
struct Deck {
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
    fn display_deck() -> Result<(), String> {
        let deck = Deck::new();
        println!("{}", deck);
        Ok(())
    }

    #[test]
    fn shuffle_deck() {
        let deck = Deck::new();
        let mut deck_to_shuffle = Deck::new();
        deck_to_shuffle.shuffle();
        assert_ne!(deck, deck_to_shuffle)
    }
}
