#![forbid(unsafe_code)]

pub mod r#trait;
use r#trait::{FairRound, Round, UnfairRound};
mod config;
mod games;
use config::{get_game, GameConfig};
type Game = Box<dyn Round>;

pub fn play_game(x: &mut Game, fair_rounds: usize, unfair_rounds: usize) -> Option<u8> {
    let mut winner = None;
    for _ in 0..fair_rounds {
        winner = Some(FairRound::play(&mut **x));
    }
    for _ in 0..unfair_rounds {
        winner = Some(UnfairRound::play(&mut **x));
    }
    winner
}

pub fn play_games(games: &Vec<(String, usize, usize)>) -> Vec<Option<u8>> {
    let mut ans: Vec<Option<u8>> = Vec::new();
    ans.reserve(games.len());
    for el in games {
        let config = serde_json::from_str::<GameConfig>(&el.0).unwrap();
        println!("{:?}", &config);
        let mut game: Game = get_game(config);
        ans.push(play_game(&mut game, el.1, el.2));
    }

    ans
}
