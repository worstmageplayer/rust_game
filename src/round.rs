use std::io::{self, Write};

use crate::deck::{Card};
use crate::player::{Player};

pub fn dealer_turn(dealer: &mut Player, deck: &mut Vec<Card>) {
    while dealer.hand_value() < 17 {
        if let Some(card) = deck.pop() {
            dealer.add_card(card);
        } else {
            break;
        }
    }
}

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

pub fn end_round(group: &Vec<Player>) {
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
