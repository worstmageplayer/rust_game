use std::io::{self, Write};

use crate::deck::{Card};
use crate::input::get_input;
use crate::player::{create_group, player, Player};

pub fn start_game() -> Vec<Player> {
    println!("Blackjack\n");

    let mut players = Vec::<Player>::new();

    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            println!("Maximum of five players allowed.");
            break;
        };

        println!("-= Player {i} =-");

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

        let create_new_player = get_input("Add player? (y/n)\n> ");
        if create_new_player != "y" {
            break;
        }
    };
    println!("\nPlayers:");
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
        if dealer.draw_card(deck).is_err() {
            break;
        };
    }
}

pub fn player_turn(player: &mut Player, deck: &mut Vec<Card>) {
    if player.is_dealer {
        println!("Error.\nDealer tried to access player_turn.");
        return;
    }
    println!("\n{}'s turn — choose an action:", player.name);
    println!("1) Hit");
    println!("2) Double Down");
    println!("3) Stand");
    println!("4) View hand");
    println!("5) View balance");
    println!("\n{}'s hand", player.name);
    player.view_hand();

    if player.view_hand_value() == 21 {
        println!("BLACKJACK! (1.5x)");
        return;
    }

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
                if player.draw_card(deck).is_err() {
                    println!("Deck is empty!");
                } else {
                    println!("{} drew {}", player.name, player.hand.last().unwrap());
                    let value = player.view_hand_value();

                    if value > 21 {
                        println!("{} busted.", player.name);
                        break;
                    } else if value == 21 {
                        println!("BLACKJACK!");
                        break;
                    } else if player.hand.len() >= 5 && value <= 21 {
                        println!("5 cards.");
                        break;
                    }
                }
            }
            "2" | "double down" | "dd" => {
                if !player.hand.len() == 2 {
                    println!("Double down not allowed.");
                    continue;
                }
                println!("{} double downs.", player.name);

                if player.draw_card(deck).is_err() {
                    println!("Deck is empty!");
                } else {
                    println!("{} drew {}", player.name, player.hand.last().unwrap());
                    player.view_hand_value();
                    player.bet_multiplier = 2.0;
                    break;
                }
            }
            "3" | "split" | "sp" => {
                if !player.hand.len() != 2 {
                    println!("Split not allowed.\nYou need two card.");
                    continue;
                }

                let first_card = player.hand.first().unwrap().rank;
                let second_card = player.hand.last().unwrap().rank;

                if first_card != second_card {
                    println!("Split not allowed.\nCards are not the same rank.");
                    continue;
                }

                println!("{} splits.", player.name);
            }
            "4" | "stand" | "s" => {
                println!("{} stands.", player.name);
                break;
            }
            "5" | "hand" => {
                println!("{}'s hand", player.name);
                player.view_hand();
                player.view_hand_value();
            }
            "6" | "balance" | "b" => {
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
            player.bet_multiplier = -player.bet_multiplier;
            "busts and loses"
        } else if player_value == 21 && player.hand.len() == 2 {
            player.bet_multiplier = 1.5;
            "wins (BLACKJACK)"
        } else if player.hand.len() >= 5 && player_value <= 21 {
            "wins"
        } else if dealer_value > 21 {
            "wins (dealer busted)"
        } else if player_value > dealer_value {
            "wins"
        } else if player_value == dealer_value {
            player.bet_multiplier = 0.0;
            "pushes (tie)"
        } else {
            player.bet_multiplier = -player.bet_multiplier;
            "loses"
        };

        println!("=> {} {}", player.name, result);

        let player_bet = player.bet * player.bet_multiplier;
        player.modify_balance(player_bet);
        player.bet_multiplier = 1.0;
    }
}
