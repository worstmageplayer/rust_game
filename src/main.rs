#![allow(dead_code)]

use rand::seq::SliceRandom;
use rand::thread_rng;

mod deck;
mod player;
mod round;
mod input;

use crate::deck::{generate_deck};
use crate::player::{find_player_mut};
use crate::round::{dealer_turn, end_round, player_turn, start_game};
use crate::input::{get_input};

fn main() {
    let mut group = start_game();

    println!("\nGenerating deck");
    let mut deck = generate_deck();

    loop {
        for player in &mut group {
            player.return_hand_to_deck(&mut deck);
        }

        println!("Shuffling the deck");
        deck.shuffle(&mut thread_rng());

        for player in &mut group {
            if let Err(e) = player.draw_card(&mut deck) {
                println!("Error: {e}");
                return;
            }
            if let Err(e) = player.draw_card(&mut deck) {
                println!("Error: {e}");
                return;
            }
        };

        for player in &mut group[1..] {
            player_turn(player, &mut deck);
        }

        dealer_turn(&mut group[0], &mut deck);

        end_round(&mut group);

        let choice = get_input("\nPlay again? (y)\nChange bet amount (b)\n> ").to_lowercase();

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
                    _ => {
                        println!("Player '{name}' not found.");
                        continue;
                    }
                };

                player.bet = loop {
                    let bet_input = get_input("Enter new bet amount: ");
                    match bet_input.parse::<f64>() {
                        Ok(b) if b > 0.0 => break b,
                        _ => println!("Invalid input. Please enter a number greater than 0."),
                    }
                };
                println!("{}'s bet has been set to ${:.2}", player.name, player.bet);
                break;
            }
        }
    }
}
