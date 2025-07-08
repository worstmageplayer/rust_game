use crate::deck::{Card, Ranks};

pub struct Player {
    pub name: String,
    pub balance: f64,
    pub bet: f64,
    pub hand: Vec<Card>,
    pub is_dealer: bool,
}

pub fn player(name: String, balance: f64, bet: f64) -> Player {
    Player {
        name,
        balance,
        bet,
        hand: Vec::<Card>::new(),
        is_dealer: false,
    }
}

pub fn create_group(players: Vec<Player>) -> Vec<Player> {
    let mut group = Vec::<Player>::new();

    let dealer = Player {
        name: "dealer".to_string(),
        balance: 0.0,
        bet: 0.0,
        hand: Vec::<Card>::new(),
        is_dealer: true,
    };

    group.push(dealer);
    group.extend(players);

    group
}

pub fn find_player_mut<'a>(group: &'a mut [Player], name: &str) -> Option<&'a mut Player> {
    group.iter_mut().find(|p| p.name == name)
}

pub fn find_player<'a>(group: &'a [Player], name: &str) -> Option<&'a Player> {
    group.iter().find(|p| p.name == name)
}

impl Player {
    pub fn view_stats(&self) {
        println!("{}'s stats: ", self.name);
        for card in &self.hand {
            println!("- {card}");
        };
        println!("Value: {}", self.hand_value());
        self.view_balance();
    }

    pub fn draw_card(&mut self, deck: &mut Vec<Card>) -> Result<(), String> {
        if let Some(card) = deck.pop() {
            self.hand.push(card);
            Ok(())
        } else {
            Err("Not enough cards in the deck".to_string())
        }
    }

    pub fn return_hand_to_deck(&mut self, deck: &mut Vec<Card>) {
        deck.append(&mut self.hand);
    }

    pub fn view_hand(&self) {
        for card in &self.hand {
            println!("- {card}");
        };
    }

    pub fn view_hand_value(&self) -> u64 {
        let value = self.hand_value();
        println!("{}'s hand value: {}", self.name, value);
        value
    }

    pub fn view_balance(&self) {
        println!("{}'s Balance: ${}", self.name, self.balance);
    }

    pub fn modify_balance(&mut self, amount: f64) {
        if self.is_dealer {
            println!("Error.\nmodify_balance used on dealer.");
            return;
        };
        self.balance += amount;
        let prefix = if amount > 0.0 {
            "+"
        } else {
            "-"
        };
        println!("\n{} {}${}", self.name, prefix, amount.abs());

    }

    pub fn hand_value(&self) -> u64 {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.hand {
            total += match card.rank {
                Ranks::Two => 2,
                Ranks::Three => 3,
                Ranks::Four => 4,
                Ranks::Five => 5,
                Ranks::Six => 6,
                Ranks::Seven => 7,
                Ranks::Eight => 8,
                Ranks::Nine => 9,
                Ranks::Ten => 10,
                Ranks::Jack => 10,
                Ranks::Queen => 10,
                Ranks::King => 10,
                Ranks::Ace => {
                    aces += 1;
                    11
                }
            }
        }

        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }

        total
    }
}
