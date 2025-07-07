#![allow(dead_code)]

use rand::seq::SliceRandom;
use rand::thread_rng;

mod deck;
mod player;
mod round;
mod input;

use crate::deck::{generate_deck};
use crate::player::{create_group, find_player_mut, player, Player};
use crate::round::{player_turn, dealer_turn, end_round};
use crate::input::{get_input};

fn main() {
    println!("Blackjack");

    let name = get_input("Enter your name: ");

    let bet: f64 = loop {
        let bet_input = get_input("Enter bet amount: ");
        match bet_input.parse::<f64>() {
            Ok(b) if b > 0.0 => break b,
            _ => println!("Invalid input. Please enter a number greater than 0."),
        }
    };
    println!("You bet: ${bet}");

    let mut players = Vec::<Player>::new();
    let player = player(name, 100_000.0, bet);
    players.push(player);

    let mut group = create_group(players);

    println!("\nGenerating deck");
    let mut deck = generate_deck();

    loop {
        for player in &mut group {
            player.return_hand_to_deck(&mut deck);
        }

        println!("Shuffling the deck");
        deck.shuffle(&mut thread_rng());

        for player in &mut group {
            player.draw_card(&mut deck);
            player.draw_card(&mut deck);
        };

        for player in &mut group[1..] {
            if player.is_dealer {
                println!("Error.\nIncorrect dealer position.");
                return;
            }
            player_turn(player, &mut deck);
        }

        dealer_turn(&mut group[0], &mut deck);

        end_round(&mut group);

        let choice = get_input("\nPlay again? (y/n)\nChange bet amount (b)\n> ").to_lowercase();

        if choice == "n" || choice == "no" {
            break;
        } else if choice == "b" {
            loop {
                let name = get_input("Enter player to change bet: ");

                let player = match find_player_mut(&mut group, &name) {
                    Some(p) if p.is_dealer => {
                        println!("You cannot modify the dealer's bet amount.");
                        continue;
                    },
                    Some(p) => p,
                    None => {
                        println!("Player '{name}' not found.");
                        continue;
                    }
                };

                let new_bet_amount: f64 = loop {
                    let bet_input = get_input("Enter new bet amount: ");
                    match bet_input.parse::<f64>() {
                        Ok(b) if b > 0.0 => break b,
                        _ => println!("Invalid input. Please enter a number greater than 0."),
                    }
                };
                player.bet = new_bet_amount;
                println!("{}'s bet has been set to ${:.2}", player.name, player.bet);
                break;
            }
        }
    }
}
