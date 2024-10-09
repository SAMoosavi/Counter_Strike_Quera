pub mod gun;
pub mod player;

pub mod game;
pub mod game_time;
pub mod setting;
pub mod team;

use gun::TypeOfGun;

use crate::game::{Game, TeamId};
use crate::game_time::GameTime;

use std::io;

fn handel(game: &mut Game, query: &[&str]) -> Result<String, String> {
    let command = query[0];
    match command {
        "ADD-USER" => {
            let name = query[1];
            let team_id = TeamId::to_enum(query[2])?;
            let time = GameTime::new_from_str(query[3]);

            game.add_player(team_id, name, &time)
        }
        "GET-MONEY" => {
            let name = query[1];
            let time = GameTime::new_from_str(query[2]);

            Ok(game.get_money_of_player(name, &time)?.to_string())
        }
        "GET-HEALTH" => {
            let name = query[1];
            let time = GameTime::new_from_str(query[2]);

            Ok(game.get_health_of_player(name, &time)?.to_string())
        }
        "TAP" => {
            let attacker = query[1];
            let attacked = query[2];
            let gun_type = TypeOfGun::to_enum(query[3])?;
            let time = GameTime::new_from_str(query[4]);

            game.tap(attacker, attacked, &gun_type, &time)
        }
        "BUY" => {
            let player = query[1];
            let gun = query[2];
            let time = GameTime::new_from_str(query[3]);

            game.buy(player, gun, &time)
        }
        "SCORE-BOARD" => {
            let time = GameTime::new_from_str(query[1]);

            Ok(game.score_board(&time))
        }
        _ => Err(format!("the command {} is not found!", command)),
    }
}

fn main() {
    let mut game = Game::new().unwrap();

    let mut number_of_round = String::new();
    io::stdin().read_line(&mut number_of_round).unwrap();
    let number_of_round: u8 = number_of_round.trim().parse().unwrap();
    for _ in 0..number_of_round {
        let mut number_of_act = String::new();
        io::stdin().read_line(&mut number_of_act).unwrap();
        let number_of_act: Vec<&str> = number_of_act.split_whitespace().collect();
        let number_of_act: u8 = number_of_act[1].trim().parse().unwrap();
        for _ in 0..number_of_act {
            let mut query = String::new();
            io::stdin().read_line(&mut query).unwrap();
            let query: Vec<&str> = query.split_whitespace().collect();
            match handel(&mut game, &query) {
                Ok(ans) => {
                    if !ans.is_empty() {
                        println!("{}", ans);
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
        println!("{}", game.end_of_round());
    }
}
