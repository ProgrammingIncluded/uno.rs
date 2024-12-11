use std::fmt;
use std::io;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg32;

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
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
pub enum CardType {
    Value,
    Reverse,
    Cancel,
    DrawTwo,
    DrawFour,
    Wild,
}

#[derive(Clone)]
pub struct Card {
    pub value: u8,
    pub color: Color,
    pub variant: CardType,
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

pub struct Game {
    pub deck: Vec<Card>,
    pub field: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
    pub players: usize,
    pub turn: usize,
    pub direction: bool,
    pub seed: u64,
    pub chainable: bool,
    pub accum: u32,
}

impl Game {
    pub fn init(players: usize, deck_size: usize, hand_size: usize, seed: u64) -> Self {
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

// Order prioritized by how cards are played.
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Variant {
    DrawDeck,
    Play,
    Reverse,
    Skip,
    Draw2,
    Draw4,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub hand_idx: usize,
    pub player_idx: usize,
    pub variant: Variant,
    pub as_color: Color, // Mainly used for declaring wild card colors.
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
    pub fn generate(&self) -> Vec<Move> {
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

    pub fn simulate(&self, picked: &Move) -> Game {
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

    pub fn playable(
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
