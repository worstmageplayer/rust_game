use std::io::{self, Write};

use crate::deck::{Card};
use crate::input::get_input;
use crate::player::{create_group, player, Player};

pub fn start_game() -> Vec<Player> {
    println!("Blackjack");

    let mut players = Vec::<Player>::new();

    let mut i = 0;
    loop {
        i += 1;
        println!("Player {i} -");

        let player_name = get_input("Enter your name: ");

        let bet: f64 = loop {
            let bet_input = get_input("Enter bet amount: ");
            match bet_input.parse::<f64>() {
                Ok(b) if b > 0.0 => break b,
                _ => println!("Invalid input. Please enter a number greater than 0."),
            }
        };
        println!("You bet ${bet}");

        let player = player(player_name, 100_000.0, bet);
        players.push(player);

        let create_new_player = get_input("Create new player? (y/n)\n> ");
        if create_new_player != "y" {
            break;
        }
    };
    println!("\nAll players created:");
    for p in &players {
        println!("{} - Bet: ${}", p.name, p.bet);
    }

    create_group(players)
}

pub fn dealer_turn(dealer: &mut Player, deck: &mut Vec<Card>) {
    if !dealer.is_dealer {
        println!("Error.\nPlayer tried to access dealer_turn.");
        return;
    }
    while dealer.hand_value() < 17 {
        if let Some(card) = deck.pop() {
            dealer.add_card(card);
        } else {
            break;
        }
    }
}

pub fn player_turn(player: &mut Player, deck: &mut Vec<Card>) {
    if player.is_dealer {
        println!("Error.\nDealer tried to access player_turn.");
        return;
    }
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
                    } else if player.hand_value() == 21 {
                        println!("BLACKJACK!");
                        break;
                    } else if player.hand.len() >= 5 && player.hand_value() <= 21 {
                        println!("5 cards.");
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

pub fn end_round(group: &mut [Player]) {
    if group.is_empty() {
        println!("No players in this round.");
        return;
    }

    let dealer = &group[0];
    let dealer_value = dealer.hand_value();
    println!("\nDealer's hand ({dealer_value}):");
    dealer.view_hand();


    for player in &mut group[1..] {
        let player_value = player.hand_value();
        println!("\n{}'s hand ({})", player.name, player_value);
        player.view_hand();

        let result = if player_value > 21 {
            (-player.bet, "busts and loses")
        } else if player_value == 21 && player.hand.len() == 2 {
            (player.bet * 1.5, "wins (BLACKJACK)")
        } else if player.hand.len() >= 5 && player_value <= 21 {
            (player.bet, "wins")
        } else if dealer_value > 21 {
            (player.bet, "wins (dealer busted)")
        } else if player_value > dealer_value {
            (player.bet, "wins")
        } else if player_value == dealer_value {
            (0.0, "pushes (tie)")
        } else {
            (-player.bet, "loses")
        };

        println!("=> {} {}", player.name, result.1);
        player.modify_balance(result.0);
    }
}
