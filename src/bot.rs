use rand::SeedableRng;
use rand::Rng;

use crate::game::Move;

#[derive(Clone, Debug, PartialEq)]
pub enum BotType {
    RANDOM,
    CONSERVATIVE
}


pub struct Bot {
    pub strategy: BotType
}

impl Bot {
    pub fn run(&self, moves: &Vec<Move>) -> usize {
        let select = match self.strategy {
            BotType::RANDOM => {
                let bot_selection = rand::thread_rng().gen_range(0..moves.len()) as usize;
                bot_selection
            },
            BotType::CONSERVATIVE => {
                // We always play number cards first and save up other special cards.
                let mut sorted_moves: Vec<(usize, Move)> = moves.clone().into_iter().enumerate().collect();
                sorted_moves.sort_by(|a, b| b.1.variant.cmp(&a.1.variant));
                sorted_moves[0].0
            }
        };
        println!("Bot has picked move: {}", select);
        return select;
    }
}
