use rand::seq::SliceRandom;
// strum is a library to more easily stringify enums (which are a big component of Rust)
use strum::{Display, IntoEnumIterator};
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Display, EnumIter)]
enum Suit {
    #[strum(to_string = "♥")]
    Hearts,
    #[strum(to_string = "♦")]
    Diamonds,
    #[strum(to_string = "♣")]
    Clubs,
    #[strum(to_string = "♠")]
    Spades
}

#[derive(Clone, Copy, PartialEq, Display, EnumIter)]
enum Rank {
    A, J, Q, K,
    #[strum(to_string = "{num}")]
    Number { num: u8 }
}

#[derive(Clone, Copy)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

impl Card {
    // Aces have multiple values, and Aces are Cards, so any Card can have multiple values.
    fn values(&self) -> Vec<u8> {
        const FACE_VALUE: u8 = 10; // every face card has a value of 10
        match self.rank {
            Rank::A => vec![1, 11],
            Rank::Number { num } => vec![num],
            _ => vec![FACE_VALUE], // every card that isn't an Ace or a Number card is a face
        }
    }
}

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        const DECK_SIZE: usize = 52;
        let mut cards = Vec::with_capacity(DECK_SIZE);
        for suit in Suit::iter() {
            // it's a bit harder to iterate over ranks due to the Number variant (which we invented to avoid manually defining Two, Three, Four ... Ten variants)
            const NUM_RANKS: usize = 13;
            let mut ranks = Vec::with_capacity(NUM_RANKS);
            for x in [Rank::A, Rank::J, Rank::K, Rank::Q] {ranks.push(x)};
            for num in 2..=10 { ranks.push(Rank::Number { num }); }
            for rank in ranks { cards.push(Card {suit, rank}); }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng())
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
    
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&self) -> u8 {
        // every Card has multiple values
        // the value of a Hand is the maximum of all POSSIBLE sums of its Cards' values that is lower than 21 (if one exists)
        let mut hand_vals: Vec<u8> = vec![0];
        for card in &self.cards {
            let mut old_vals = vec![];
            while let Some(old_val) = hand_vals.pop() {
                old_vals.push(old_val);
            }
            for new_val in card.values() {
                for old_val in &old_vals {
                    if new_val + old_val <= 21 { hand_vals.push(new_val + old_val); }
                }
            }
            if hand_vals.is_empty() { return 0; }
        }
        match hand_vals.iter().max() {
            Some(max) => *max,
            _ => 0,
        }
    }

    pub fn is_blackjack(&self) -> bool { self.value() == 21 && self.cards.len() == 2 }

    // necessary for dealer in first round
    pub fn last_card(&self) -> Option<Card> {
        self.cards.last().copied()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cards: Vec<String> = self.cards.iter().map(|card| card.to_string()).collect();
        write!(f, "{} (Value: {})", cards.join(", "), self.value())
    }
}
