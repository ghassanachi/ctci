use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt::Display;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

pub struct BlackJack {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub dealer: Player,
}

#[derive(PartialEq, Eq)]
enum PlayerStatus {
    Playing,
    Staying,
    Busted,
}

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    status: PlayerStatus,
}

pub struct Deck(Vec<Card>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Card {
    Ace(Suit),
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ten(Suit),
    Nine(Suit),
    Eight(Suit),
    Seven(Suit),
    Six(Suit),
    Five(Suit),
    Four(Suit),
    Three(Suit),
    Two(Suit),
}

impl BlackJack {
    pub fn new() -> Self {
        let mut result = Self {
            deck: Deck::new(),
            players: Vec::new(),
            dealer: Player::dealer(),
        };
        let player_number;

        loop {
            println!("Enter number of players between 1 and 10:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let n: Result<u8, _> = input.trim().parse();
            if let Ok(num) = n {
                if num == 0 || num > 10 {
                    continue;
                }
                player_number = num;
                break;
            }
        }

        for _ in 0..player_number {
            result.players.push(Player::new());
        }
        result
    }

    pub fn run(&mut self) {
        println!(concat!(
            "++++++++++++++++++++++++++++++++++\n",
            "+  Black Jack Game is starting!  +\n",
            "++++++++++++++++++++++++++++++++++\n",
        ));

        self.deck.shuffle();

        self.dealer.hit(self.deck.draw());
        self.dealer.hit(self.deck.draw());

        for player in &mut self.players {
            player.hit(self.deck.draw());
            player.hit(self.deck.draw());
        }

        for player in &mut self.players {
            while player.status == PlayerStatus::Playing {
                println!("\n{}\n", player.to_string());
                print!("Available Actions: Hit (h), Stay (s), Quit (q): ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read line");
                input.pop();
                match input.trim() {
                    "h" | "H" => {
                        let card = self.deck.draw();
                        println!("{} is dealt: {}", player.name, card);
                        let totals = player.hit(card);
                        let min = totals.iter().min().unwrap();
                        if min > &21 {
                            println!("{} Busted with {}", player.name, min);
                            player.status = PlayerStatus::Busted;
                        }
                    }
                    "s" | "S" => {
                        println!("{} is staying at {}", player.name, player.best_total());
                        player.status = PlayerStatus::Staying;
                    }
                    "q" | "Q" => {
                        println!("Thanks for playing!");
                        return;
                    }
                    _ => {}
                }
            }
        }

        while self.dealer.status == PlayerStatus::Playing {
            println!("\n{}\n", self.dealer.dealer_to_string());

            let dealer_val = self.dealer.dealer_total();

            if dealer_val > 21 {
                println!("Dealer Busted!");
                self.dealer.status = PlayerStatus::Busted;
            } else if dealer_val >= 17 {
                println!("Dealer Staying!");
                self.dealer.status = PlayerStatus::Staying;
                break;
            } else {
                let card = self.deck.draw();
                self.dealer.hit(card);
                println!("Dealer takes a hit: {}", card)
            }
        }

        if self.dealer.status == PlayerStatus::Busted {
            let winners: Vec<_> = self
                .players
                .iter()
                .filter_map(|player| {
                    if player.status == PlayerStatus::Staying {
                        return Some(player.name.clone());
                    }
                    None
                })
                .collect();

            println!("Winners: {}", winners.join(", "));
            return;
        }

        let dealer_val = self.dealer.dealer_total();
        let winners: Vec<_> = self
            .players
            .iter()
            .filter_map(|player| {
                if player.status == PlayerStatus::Staying && player.best_total() >= dealer_val {
                    return Some(player.name.clone());
                }
                None
            })
            .collect();
        if winners.len() == 0 {
            println!("The Dealer Won!");
        } else {
            println!("Winners: {}", winners.join(", "));
        }
    }
}

impl Player {
    pub fn new() -> Self {
        println!("Enter player's name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("failed to read name");
        // remove new line
        name.pop();
        Self {
            name,
            hand: Vec::new(),
            status: PlayerStatus::Playing,
        }
    }

    fn dealer() -> Self {
        Self {
            name: String::from("Dealer"),
            hand: Vec::new(),
            status: PlayerStatus::Playing,
        }
    }

    pub fn dealer_total(&self) -> u8 {
        let mut has_ace = false;
        let mut total = 0;
        for card in &self.hand {
            match card {
                Card::Ace(_) if has_ace => {
                    total += card.val(false);
                }
                Card::Ace(_) if !has_ace => {
                    has_ace = true;
                    if total + card.val(true) <= 21 {
                        total += card.val(true)
                    } else {
                        total = card.val(false);
                    }
                }
                _ => total += card.val(false),
            }
        }
        total
    }

    pub fn possible_totals(&self) -> Vec<u8> {
        let mut result: HashSet<u8> = HashSet::from_iter([0]);
        for card in &self.hand {
            let drain_iter: Vec<_> = result.drain().collect();
            for current_total in drain_iter {
                result.insert(current_total + card.val(false));
                if let Card::Ace(_) = card {
                    result.insert(current_total + card.val(true));
                }
            }
        }
        result.into_iter().collect()
    }

    pub fn best_total(&self) -> u8 {
        let totals = self.possible_totals();
        let mut best = 0;
        for total in totals {
            if total > best && total <= 21 {
                best = total
            }
        }
        best
    }

    pub fn clear(&mut self) {
        self.hand.clear()
    }

    pub fn hit(&mut self, card: Card) -> Vec<u8> {
        self.hand.push(card);
        self.possible_totals()
    }

    pub fn to_string(&self) -> String {
        let hand = self
            .hand
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<_>>()
            .join(" ");
        let totals = self
            .possible_totals()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        format!(
            "Player {}'s Turn:\nHand: {}\nTotals: {{{}}}",
            self.name, hand, totals
        )
    }

    pub fn dealer_to_string(&self) -> String {
        let hand = self
            .hand
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<_>>()
            .join(" ");
        let total = self.dealer_total();
        format!("Dealer's Turn:\nHand: {}\nTotal: {{{}}}", hand, total)
    }
}

impl Card {
    pub fn val(&self, ace_high: bool) -> u8 {
        match self {
            Card::Nine(_) => 9,
            Card::Eight(_) => 8,
            Card::Seven(_) => 7,
            Card::Six(_) => 6,
            Card::Five(_) => 5,
            Card::Four(_) => 4,
            Card::Three(_) => 3,
            Card::Two(_) => 2,
            Card::Ace(_) if !ace_high => 1,
            Card::Ace(_) if ace_high => 11,
            _ => 10,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Deck {
    pub fn new() -> Self {
        let cards = (1..=52).fold(Vec::with_capacity(52), |mut deck, num| {
            deck.push(((num % 13) + 1, (num % 4) + 1).into());
            deck
        });
        Self(cards)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.0.shuffle(&mut rng);
    }

    /// Add a new deck when we run out, this is done for simplicity, but if we wanted a more robust
    /// system, we could maintain the list of cards in the discard pile and reshuffle them in while
    /// we still had some to draw to make it harder to know what is at the bottom of the draw pile
    pub fn draw(&mut self) -> Card {
        if let Some(card) = self.pop() {
            return card;
        }
        *self = Self::new();
        self.pop().unwrap()
    }
}

impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (self
                .iter()
                .map(|card| format!("{}", card))
                .collect::<Vec<_>>())
            .join(" ")
        )
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Ace(suit) => write!(f, "A{}", suit),
            Card::King(suit) => write!(f, "K{}", suit),
            Card::Queen(suit) => write!(f, "Q{}", suit),
            Card::Jack(suit) => write!(f, "J{}", suit),
            Card::Ten(suit) => write!(f, "10{}", suit),
            Card::Nine(suit) => write!(f, "9{}", suit),
            Card::Eight(suit) => write!(f, "8{}", suit),
            Card::Seven(suit) => write!(f, "7{}", suit),
            Card::Six(suit) => write!(f, "6{}", suit),
            Card::Five(suit) => write!(f, "5{}", suit),
            Card::Four(suit) => write!(f, "4{}", suit),
            Card::Three(suit) => write!(f, "3{}", suit),
            Card::Two(suit) => write!(f, "2{}", suit),
        }
    }
}

impl From<u8> for Suit {
    fn from(num: u8) -> Self {
        match num {
            1 => Suit::Spade,
            2 => Suit::Heart,
            3 => Suit::Diamond,
            4 => Suit::Club,
            _ => panic!("Suit number should between 1 and 4"),
        }
    }
}

impl From<(u8, u8)> for Card {
    fn from(card: (u8, u8)) -> Self {
        let suit: Suit = card.1.into();
        match card.0 {
            1 => Card::Ace(suit),
            2 => Card::Two(suit),
            3 => Card::Three(suit),
            4 => Card::Four(suit),
            5 => Card::Five(suit),
            6 => Card::Six(suit),
            7 => Card::Seven(suit),
            8 => Card::Eight(suit),
            9 => Card::Nine(suit),
            10 => Card::Ten(suit),
            11 => Card::Jack(suit),
            12 => Card::Queen(suit),
            13 => Card::King(suit),
            _ => panic!("Card valud should be between 1 and 13"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spade => write!(f, "♠"),
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Club => write!(f, "♣"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_of_cards_1() {
        let suit: Suit = 1u8.into();
        let card: Card = (10u8, 2u8).into();
        assert_eq!(suit, Suit::Spade);
        assert_eq!(card, Card::Ten(Suit::Heart));
    }
}
