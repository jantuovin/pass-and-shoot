use crate::game_logic;
use crate::game_play;
use rand::Rng;
use std::{thread, time};

pub fn read_input(game: &game_logic::Game) -> String {
    thread::sleep(time::Duration::from_millis(1000));

    let player_with_ball = match game.players.get(&game.name_of_player_with_ball) {
        Some(p) => p,
        _ => return String::from(game_play::allowed_command::QUIT),
    };

    let possible_team_mates_to_pass: Vec<&game_logic::Player> = game
        .players
        .values()
        .filter(|p| p.team == player_with_ball.team)
        .filter(|p| match p.team.as_str() {
            game_logic::team_name::R => player_with_ball.pos_y_axis < p.pos_y_axis,
            _ => p.pos_y_axis < player_with_ball.pos_y_axis,
        })
        .collect();

    if possible_team_mates_to_pass.len() > 0 && (game.round / game.total_rounds) as f32 <= 0.90 {
        let i = rand::thread_rng().gen_range(0..(possible_team_mates_to_pass.len() - 1));
        return format!(
            "{} {}",
            game_play::allowed_command::PASS,
            possible_team_mates_to_pass[i].name
        );
    }

    match player_with_ball.team.as_str() {
        game_logic::team_name::R => format!(
            "{} {}",
            game_play::allowed_command::SHOOT,
            game_logic::team_name::B
        ),
        _ => format!(
            "{} {}",
            game_play::allowed_command::SHOOT,
            game_logic::team_name::R
        ),
    }
}
