use deck::{Value, Card, Deck};

fn card_value(card: &Card) -> u8 {
    match card.value {
        Value::Ace => 11,
        Value::Two => 2,
        Value::Three => 3,
        Value::Four => 4,
        Value::Five => 5,
        Value::Six => 6,
        Value::Seven => 7,
        Value::Eight => 8,
        Value::Nine => 9,
        Value::Ten => 10,
        Value::Jack => 10,
        Value::Queen => 10,
        Value::King => 10
    }
}

fn draw_card(deck: &mut Deck) -> Card {
    match deck.draw() {
        Some(card) => card,
        None => {
            deck.reset();
            deck.shuffle();
            deck.draw().unwrap()
        }
    }
}

pub trait Hand {
    fn add_card(&mut self, card: Card);
    fn get_value(&self) -> u8;
}

impl Hand for Vec<Card> {
    fn add_card(&mut self, card: Card) {
        self.push(card);
    }

    fn get_value(&self) -> u8 {
        let mut value = 0;
        for card in self {
            value += card_value(card);
        }
        for card in self {
            if card.value == Value::Ace && value > 21 {
                value -= 10;
            }
        }
        value
    }
}

struct Player {
    name: String,
    hand: Vec<Card>
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            hand: Vec::new()
        }
    }

    fn hit(&mut self, deck: &mut Deck) {
        self.hand.add_card(draw_card(deck));
    }

    fn show_hand(&self) {
        println!("{}'s hand:", self.name);
        for card in &self.hand {
            println!("{}: {}", card, card_value(card));
        }
        println!("Value: {}\n", self.hand.get_value());
    }

    fn show_one_card(&self) {
        println!("{}'s first card:", self.name);
        println!("{}: {}\n", self.hand[0], card_value(&self.hand[0]));
    }
}

fn play_round(player: &mut Player, house: &mut Player, deck: &mut Deck) {
    player.hand.clear();
    house.hand.clear();

    player.hit(deck);
    house.hit(deck);
    player.hit(deck);
    house.hit(deck);
    player.show_hand();
    house.show_one_card();
    if house.hand[0].value == Value::Ace {
        if house.hand.get_value() == 21 {
            if player.hand.get_value() != 21 {
                println!("House wins.");
            } else {
                println!("Push!");
            }
            return;
        } else {
            println!("House does not have blackjack.");
        }
    }

    loop {
        println!("Hit or stand? (h/s)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "h" {
            player.hit(deck);
            player.show_hand();
            if player.hand.get_value() > 21 {
                println!("Bust! You lose.");
                return;
            }
        } else if input.trim().to_lowercase() == "s" {
            break;
        } else {
            println!("Invalid input.");
        }
    }

    loop {
        if house.hand.get_value() < 17 {
            house.hit(deck);
            println!("House hits.\n");
            house.show_hand();
            if house.hand.get_value() > 21 {
                println!("House busts! Player wins.");
                return;
            }
        } else {
            println!("Dealer stands.");
            break;
        }
    }

    if player.hand.get_value() > house.hand.get_value() {
        println!("Player wins.");
    } else if player.hand.get_value() < house.hand.get_value() {
        println!("House wins.");
    } else {
        println!("Push!");
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let mut player = Player::new("Player".to_string());
    let mut house = Player::new("Dealer".to_string());

    loop {
        play_round(&mut player, &mut house, &mut deck);
        println!("Play again? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "n" {
            break;
        }
    }
}
