use clap::Parser;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::Rng;
use rand_pcg::Pcg32;
use std::fmt;
use std::io;

#[derive(Clone, Debug, PartialEq)]
enum Color {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    WILD,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::RED => write!(f, "R"),
            Color::BLUE => write!(f, "B"),
            Color::GREEN => write!(f, "G"),
            Color::YELLOW => write!(f, "Y"),
            Color::WILD => write!(f, "*"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CardType {
    Value,
    Reverse,
    Cancel,
    DrawTwo,
    DrawFour,
    Wild,
}

#[derive(Clone)]
struct Card {
    value: u8,
    color: Color,
    variant: CardType,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            CardType::Value => {
                write!(f, "V{}{}", self.value, self.color)
            }
            CardType::DrawTwo => {
                write!(f, "D2{}", self.color)
            }
            CardType::DrawFour => {
                write!(f, "D4{}", self.color)
            }
            CardType::Wild => {
                write!(f, "*-{}", self.color)
            }
            CardType::Cancel => {
                write!(f, "S-{}", self.color)
            }
            CardType::Reverse => {
                write!(f, "R-{}", self.color)
            }
        }
    }
}

struct Game {
    deck: Vec<Card>,
    field: Vec<Card>,
    hands: Vec<Vec<Card>>,
    players: usize,
    turn: usize,
    direction: bool,
    seed: u64,
    chainable: bool,
    accum: u32,
}

impl Game {
    fn init(players: usize, deck_size: usize, hand_size: usize, seed: u64) -> Self {
        let mut rng = Pcg32::seed_from_u64(seed);
        let seed = seed + 1;
        let mut cards = vec![];
        let mut hands = vec![vec![]; players];

        // Basic value adds
        for i in 1..10 {
            for _ in 0..2 {
                for color in [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW] {
                    cards.push(Card {
                        value: i as u8,
                        color,
                        variant: CardType::Value,
                    })
                }
            }
        }
  
        // Add the zeroes
        for color in [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW] {
            cards.push(Card {
                value: 0,
                color,
                variant: CardType::Value,
            })
        }
  
        for _ in 0..2 {
            for color in [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW] {
                cards.push(Card {
                    value: 0,
                    color: color.clone(),
                    variant: CardType::Reverse,
                });
                cards.push(Card {
                    value: 0,
                    color: color.clone(),
                    variant: CardType::Cancel,
                });
                cards.push(Card {
                    value: 0,
                    color: color.clone(),
                    variant: CardType::DrawTwo,
                });
            }
        }
  
        for _ in 0..4 {
            cards.push(Card {
                value: 0,
                color: Color::WILD,
                variant: CardType::DrawFour,
            });
            cards.push(Card {
                value: 0,
                color: Color::WILD,
                variant: CardType::Wild,
            });
        }

        for _ in 0..deck_size - 1 {
            cards.append(&mut cards.clone());
        }

        cards.shuffle(&mut rng);
        let top_card = cards.remove(cards.len() - 1);
        let field = vec![top_card];

        // Fill hands
        for i in 0..players {
            for _ in 0..hand_size {
                hands[i].push(cards.remove(cards.len() - 1));
            }
        }

        return Game {
            hands,
            players,
            field,
            seed,
            deck: cards,
            turn: 0,
            direction: false,
            chainable: false,
            accum: 0,
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Variant {
    Play,
    Skip,
    DrawDeck,
    Draw4,
    Draw2,
    Reverse,
}

#[derive(Clone, Debug, PartialEq)]
struct Move {
    hand_idx: usize,
    player_idx: usize,
    variant: Variant,
    as_color: Color, // Mainly used for declaring wild card colors.
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            Variant::Play => write!(f, "H{}{}", self.hand_idx, self.as_color),
            Variant::Skip => write!(f, "S{}", self.as_color),
            Variant::DrawDeck => write!(f, "D"),
            Variant::Draw4 => write!(f, "D4->{}", self.as_color),
            Variant::Draw2 => write!(f, "D2"),
            Variant::Reverse => write!(f, "R"),
        }
    }
}

impl Game {
    fn generate(&self) -> Vec<Move> {
        let hand = &self.hands[self.turn];
        let top = &self.field[self.field.len() - 1];

        let mut moves = vec![];
        for i in 0..hand.len() {
            if let Some(m) = Game::playable(top, &hand[i], i, self.turn, self.chainable) {
                // Expand wild moves if available.
                if m.as_color == Color::WILD {
                    for c in [Color::RED, Color::YELLOW, Color::BLUE, Color::GREEN] {
                        let mut new_m = m.clone();
                        new_m.as_color = c;
                        moves.push(new_m);
                    }
                } else {
                    moves.push(m);
                }
            }
        }

        if moves.len() == 0 && hand.len() != 0 {
            moves.push(Move {
                hand_idx: 0,
                player_idx: self.turn,
                variant: Variant::DrawDeck,
                as_color: Color::WILD,
            })
        }
        return moves;
    }

    fn simulate(&self, picked: &Move) -> Game {
        let mut hands = self.hands.clone();
        let mut deck = self.deck.clone();
        let mut field = self.field.clone();
        let mut seed = self.seed;
        let mut accum = self.accum;
        let mut chainable = self.chainable;

        let direction = self.direction
            ^ if picked.variant == Variant::Reverse {
                true
            } else {
                false
            };

        let sign = if !direction { 1 } else { -1 };
        let turn: i32 = match picked.variant {
            Variant::Skip => 2,
            _ => 1,
        };
        let turn: i32 = ((self.turn as i32) + turn * sign).rem_euclid(self.players as i32);
        let turn = turn as usize;

        match picked.variant {
            Variant::DrawDeck => {
                // Draw deck could resolve a chained draw, in which case, we have to draw more.
                // Could just be normal draw, in which case, we just draw one.
                if accum <= 0 {
                    accum = 1
                }
                for _ in 0..accum {
                    let removed_card = deck.remove(deck.len() - 1);
                    hands[picked.player_idx].push(removed_card);
                }

                chainable = false;
                accum = 0;
            },
            Variant::Draw2 | Variant::Draw4 => {
                accum += if picked.variant == Variant::Draw2 { 2 } else { 4 };
                chainable = true;
                // slow copy and delete vs resized to perfection.
                let mut removed_card = hands[picked.player_idx].remove(picked.hand_idx);
                removed_card.color = picked.as_color.clone(); // Mainly for wild cards.
                field.push(removed_card);
            },
            _ => {
                // slow copy and delete vs resized to perfection.
                let mut removed_card = hands[picked.player_idx].remove(picked.hand_idx);
                removed_card.color = picked.as_color.clone(); // Mainly for wild cards.
                field.push(removed_card);
            }
        }

        // If there are no more cards, we need to reshuffle the deck from the field first.
        if deck.len() == 0 {
            let mut rng = Pcg32::seed_from_u64(seed);
            let last_card = field.remove(field.len() - 1);

            deck = field;
            deck.shuffle(&mut rng);
            field = vec![last_card];
            seed += 1;
        }

        return Game {
            deck,
            hands,
            turn,
            field,
            seed,
            chainable,
            direction,
            accum,
            players: self.players,
        };
    }

    fn playable(
        top_of_deck: &Card,
        playing: &Card,
        hand_idx: usize,
        player_idx: usize,
        chainable: bool,
    ) -> Option<Move> {
        // Check if chainable. If not, then we can only play if variants match.
        if top_of_deck.variant != playing.variant && chainable
            && (top_of_deck.variant == CardType::DrawTwo
                || top_of_deck.variant == CardType::DrawFour)
        {
            return None;
        }
        // Simplify the logic a bit.
        let mut move_variant = Variant::Skip;
        match playing.variant {
            CardType::Value => {
                if top_of_deck.color == playing.color
                    || (top_of_deck.variant == playing.variant
                        && top_of_deck.value == playing.value)
                {
                    return Some(Move {
                        hand_idx,
                        player_idx,
                        variant: Variant::Play,
                        as_color: playing.color.clone(),
                    });
                }
            }
            CardType::DrawTwo => {
                move_variant = Variant::Draw2;
            }
            CardType::DrawFour => {
                move_variant = Variant::Draw4;
            }
            CardType::Reverse => {
                move_variant = Variant::Reverse;
            }
            CardType::Cancel => {
                // Skip is already resolved by simulation as it is unconditional.
                move_variant = Variant::Skip;
            }
            CardType::Wild => {
                move_variant = Variant::Play;
            }
        }

        if playing.color == Color::WILD || playing.color == top_of_deck.color {
            return Some(Move {
                hand_idx,
                player_idx,
                variant: move_variant,
                as_color: playing.color.clone(),
            });
        }
        return None;
    }
}

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
        println!(
            "{}",
            Vec::from_iter(moves.iter().enumerate().map(|(idx, i)| format!(
                "{}: {}",
                idx,
                i.to_string()
            )))
            .join(", ")
        );
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
            // Random selection
            let bot_selection = rand::thread_rng().gen_range(0..moves.len()) as usize;
            println!("Bot has picked move: {}", bot_selection);
            bot_selection
        };
        println!();

        game = game.simulate(&moves[select_idx]);
    }
}
