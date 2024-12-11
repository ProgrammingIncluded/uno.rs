use rand::SeedableRng;
use rand::Rng;

use crate::game::Move;

#[derive(Clone, Debug, PartialEq)]
pub enum BotType {
    RANDOM,
    GREEDY
}


pub struct Bot {
    pub strategy: BotType
}

impl Bot {
    pub fn run(&self, moves: &Vec<Move>) -> usize {
        let bot_selection = rand::thread_rng().gen_range(0..moves.len()) as usize;
        println!("Bot has picked move: {}", bot_selection);
        return bot_selection;
    }
}
