#![allow(dead_code)]

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
    println!("Generating deck");
    let mut deck = generate_deck();
    println!("Shuffling the deck");
    deck.shuffle(&mut thread_rng());

    let mut players = Vec::<Player>::new();

    let pslhj = player("slhj");
    players.push(pslhj);

    let mut group = create_group(players);

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

    end_round(group);
}
