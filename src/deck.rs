use std::fmt;

#[derive(Clone, Copy)]
pub enum Suits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl fmt::Display for Suits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Suits::Spades => "Spades",
            Suits::Hearts => "Hearts",
            Suits::Clubs => "Clubs",
            Suits::Diamonds => "Diamonds",
        };
        write!(f, "{name}")
    }
}

#[derive(Clone, Copy)]
pub enum Ranks {
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
    King,
}

impl fmt::Display for Ranks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Ranks::Ace => "Ace",
            Ranks::Two => "Two",
            Ranks::Three => "Three",
            Ranks::Four => "Four",
            Ranks::Five => "Five",
            Ranks::Six => "Six",
            Ranks::Seven => "Seven",
            Ranks::Eight => "Eight",
            Ranks::Nine => "Nine",
            Ranks::Ten => "Ten",
            Ranks::Jack => "Jack",
            Ranks::Queen => "Queen",
            Ranks::King => "King",
        };
        write!(f, "{name}")
    }
}

pub struct Card {
    pub suit: Suits,
    pub rank: Ranks,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

pub fn card(suit: Suits, rank: Ranks) -> Card {
    Card { suit, rank, }
}

pub fn h1(suit: Suits) -> Vec<Card> {
    vec![
        card(suit, Ranks::Ace),
        card(suit, Ranks::Two),
        card(suit, Ranks::Three),
        card(suit, Ranks::Four),
        card(suit, Ranks::Five),
        card(suit, Ranks::Six),
        card(suit, Ranks::Seven),
        card(suit, Ranks::Eight),
        card(suit, Ranks::Nine),
        card(suit, Ranks::Ten),
        card(suit, Ranks::Jack),
        card(suit, Ranks::Queen),
        card(suit, Ranks::King),
    ]
}

pub fn generate_deck() -> Vec<Card> {
    let mut deck = h1(Suits::Spades);
    deck.extend(h1(Suits::Hearts));
    deck.extend(h1(Suits::Clubs));
    deck.extend(h1(Suits::Diamonds));

    deck
}
