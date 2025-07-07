use crate::deck::{Card, Ranks};

pub struct Player {
    pub name: String,
    pub balance: f64,
    pub hand: Vec<Card>,
    pub is_dealer: bool,
}

pub fn player(name: &str) -> Player {
    Player {
        name: name.to_string(),
        balance: 100.0,
        hand: Vec::<Card>::new(),
        is_dealer: false,
    }
}

pub fn create_group(players: Vec<Player>) -> Vec<Player> {
    let mut group = Vec::<Player>::new();

    let dealer = Player {
        name: "dealer".to_string(),
        balance: 0.0,
        hand: Vec::<Card>::new(),
        is_dealer: true,
    };

    group.push(dealer);
    group.extend(players);

    group
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

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }

    pub fn view_hand(&self) {
        for card in &self.hand {
            println!("- {card}");
        };
    }

    pub fn view_hand_value(&self) {
        println!("{}'s hand value: {}", self.name, self.hand_value());
    }

    pub fn add_balance(&mut self, amount: f64) {
        if self.is_dealer { return; };
        self.balance += amount;
    }

    pub fn sub_balance(&mut self, amount: f64) {
        if self.is_dealer { return; };
        self.balance -= amount;
    }

    pub fn view_balance(&self) {
        println!("{}'s Balance: ${}", self.name, self.balance);
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

    pub fn draw_card(&mut self, deck: &mut Vec<Card>) {
        if let Some(card) = deck.pop() {
            self.hand.push(card);
        }
    }
}

pub fn dealer_turn(dealer: &mut Player, deck: &mut Vec<Card>) {
    while dealer.hand_value() < 17 {
        if let Some(card) = deck.pop() {
            dealer.add_card(card);
        } else {
            break;
        }
    }
}

use std::io::{self, Write};

pub fn player_turn(player: &mut Player, deck: &mut Vec<Card>) {
    println!("\n{}'s turn — choose an action:", player.name);
    println!("1) Hit");
    println!("2) Stand");
    println!("3) View hand");
    println!("4) View balance");
    println!("\n{}'s hand", player.name);
    player.view_hand();
    player.view_hand_value();

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }

        match input.trim() {
            "1" | "hit" | "h" => {
                if let Some(card) = deck.pop() {
                    println!("You drew: {card}");
                    player.add_card(card);
                    player.view_hand_value();

                    if player.hand_value() > 21 {
                        println!("{} busted.", player.name);
                        break;
                    }
                } else {
                    println!("Deck is empty!");
                }
            }
            "2" | "stand" | "s" => {
                println!("{} stands.", player.name);
                break;
            }
            "3" | "hand" => {
                println!("{}'s hand", player.name);
                player.view_hand();
                player.view_hand_value();
            }
            "4" | "balance" => {
                println!("Your balance: ${}", player.balance);
            }
            _ => {
                println!("Invalid option. Try again.");
            }
        }
    }
}

pub fn end_round(group: Vec<Player>) {
    if group.is_empty() {
        println!("No players in this round.");
        return;
    }

    let dealer = &group[0];
    let dealer_value = dealer.hand_value();
    println!("\nDealer's hand ({dealer_value}):");
    dealer.view_hand();

    for player in &group[1..] {
        let player_value = player.hand_value();
        println!("\n{}'s hand ({})", player.name, player_value);
        player.view_hand();

        let result = if player_value > 21 {
            "busts and loses"
        } else if dealer_value > 21 {
            "wins (dealer busted)"
        } else if player_value > dealer_value {
            "wins"
        } else if player_value == dealer_value {
            "pushes (tie)"
        } else {
            "loses"
        };

        println!("=> {} {}", player.name, result);
    }
}
