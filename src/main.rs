use clap::Parser;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::Rng;
use rand_pcg::Pcg32;
use std::fmt;
use std::io;

mod game;
use crate::game::{Game, Variant};

mod bot;
use crate::bot::{Bot, BotType};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Number of players.", default_value_t = 2)]
    players: usize,

    #[arg(
        short,
        long,
        help = "Number of uno decks to play with, there are 108 cards per game.",
        default_value_t = 1
    )]
    num_decks: usize,

    #[arg(
        short = 'x',
        long,
        help = "Hand size during the game.",
        default_value_t = 7
    )]
    hand_size: usize,

    #[arg(short, long, help = "Seed to play the game.", default_value_t = 0)]
    seed: u64,

    #[arg(long, help = "Select a player to play as otherwise the game is simulated.", default_value_t = -1)]
    play_as: i32,
}

fn main() {
    let args = Args::parse();

    if args.num_decks <= 0 {
        panic!("Must play with atleast one deck.")
    } else if args.players <= 1 {
        panic!("Must be playing with atleast one player.")
    }

    let total_cards_per_deck = 108;
    let mut num_decks = args.num_decks;
    if num_decks * total_cards_per_deck <= args.players * args.hand_size {
        println!("Number of players exceed number of cards available if each player has {} cards per hand.", args.hand_size);
        num_decks = (args.players * args.hand_size) / total_cards_per_deck + 1;
        println!(
            "Auto scaling num of decks to {} for {} players.",
            num_decks, args.players
        );
    }

    let mut game = Game::init(args.players, num_decks, args.hand_size, args.seed);
    let mut bot = Bot { strategy: BotType:: CONSERVATIVE };
    loop {
        let moves = game.generate();
        let empty = game.hands.iter().enumerate().find(|&(_, m)| m.len() == 0);
        if let Some((idx, _)) = empty {
            println!("Player {} has won the game!", idx);
            break;
        }
        println!("--- Playing as Hand #{} ---", game.turn);
        println!("Field Top: {}", game.field[game.field.len() - 1]);
        println!("Cards in Deck: {}", game.deck.len());
        println!("Cards in Field: {}", game.field.len());
        if game.accum > 1 {
            println!("Cards to Draw: {}", game.accum);
        }
        println!();

        for h in 0..game.players {
            println!(
                "Hand #{}: {}",
                h,
                Vec::from_iter(game.hands[h].iter().map(|i| i.to_string())).join(", ")
            );
        }
        println!();
        let mut result = "".to_string();
        for (idx, m) in moves.iter().enumerate() {
            // For display, we resolve hand values to make it easier to interact with.
            if m.variant == Variant::Play {
                let mut card = game.hands[m.player_idx][m.hand_idx].clone();
                card.color = m.as_color.clone();
                result.push_str(&format!("{}: {} ", idx, card));
            } else {
                result.push_str(&format!("{}: {} ", idx, m));
            }
        }
        println!("{}", result);
        println!();

        let select_idx = if args.play_as == game.turn as i32 {
            let mut final_value: usize = 0;
            loop {
                println!("Input a valid move from 0 - {}", moves.len() - 1);
                let mut input_text = String::new();
                if let Err(_) = io::stdin().read_line(&mut input_text) {
                    continue;
                }

                if let Ok(v) = input_text.trim().parse::<usize>() {
                    if final_value >= moves.len() {
                        println!("Value must be between 0 - {}", moves.len() - 1);
                        continue;
                    }
                    final_value = v;
                    break;
                } else {
                    continue;
                }
            }
            final_value
        } else {
            bot.run(&moves)
        };
        println!();

        game = game.simulate(&moves[select_idx]);
    }
}
