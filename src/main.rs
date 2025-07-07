#![allow(dead_code)]

use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::thread_rng;

mod deck;
use crate::deck::{generate_deck};

mod player;
use crate::player::{create_group, player, Player};

mod round;
use crate::round::{player_turn, dealer_turn, end_round};

fn main() {
    println!("Blackjack");

    let mut input = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let name = input.trim().to_string();

    let mut bet: f64;
    loop {
        print!("Enter your bet: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(b) if b > 0.0 => {
                bet = b;
                break;
            }
            _ => {
                println!("Invalid input. Please enter a number greater than 0.");
            }
        }
    }
    println!("You bet: ${bet}");

    let mut players = Vec::<Player>::new();
    let player = player(name);
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
            for _ in 0..2 {
                player.draw_card(&mut deck);
            }
        };

        for player in &mut group {
            if !player.is_dealer {
                player_turn(player, &mut deck);
            }
        }

        dealer_turn(&mut group[0], &mut deck);

        end_round(&mut group);

        print!("\nPlay again? (y/n): ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice = input.trim().to_lowercase();

        if choice == "n" || choice == "no" {
            break;
        }
    }
}
