use rand::Rng;
use std::collections::HashMap;

pub mod team_name {
    pub const R: &str = "R";
    pub const B: &str = "B";
}

pub mod player_name {
    pub const R_GK: &str = "R_GK";
    pub const R_LD: &str = "R_LD";
    pub const R_RD: &str = "R_RD";
    pub const R_MF: &str = "R_MF";
    pub const R_LF: &str = "R_LF";
    pub const R_RF: &str = "R_RF";
    pub const B_GK: &str = "B_GK";
    pub const B_LD: &str = "B_LD";
    pub const B_RD: &str = "B_RD";
    pub const B_MF: &str = "B_MF";
    pub const B_LF: &str = "B_LF";
    pub const B_RF: &str = "B_RF";
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub pos_x_axis: u8,
    pub pos_y_axis: u8,
}

impl Player {
    pub fn parse(name: String, team: String, pos_x_axis: u8, pos_y_axis: u8) -> Player {
        Player {
            name,
            team,
            pos_x_axis,
            pos_y_axis,
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub timestamp: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Game {
    pub players: HashMap<String, Player>,
    pub goals: HashMap<String, u8>,
    pub name_of_player_with_ball: String,
    pub events: Vec<Event>,
    pub round: u8,
    pub total_rounds: u8,
}

impl Game {
    pub fn new() -> Game {
        let mut players = HashMap::new();
        let mut insert_player = |p: Player| {
            players.insert(p.name.clone(), p);
        };

        // Red team
        insert_player(Player::parse(
            player_name::R_GK.to_string(),
            team_name::R.to_string(),
            1,
            0,
        ));
        insert_player(Player::parse(
            player_name::R_RD.to_string(),
            team_name::R.to_string(),
            0,
            1,
        ));
        insert_player(Player::parse(
            player_name::R_LD.to_string(),
            team_name::R.to_string(),
            2,
            1,
        ));
        insert_player(Player::parse(
            player_name::R_MF.to_string(),
            team_name::R.to_string(),
            1,
            3,
        ));
        insert_player(Player::parse(
            player_name::R_RF.to_string(),
            team_name::R.to_string(),
            0,
            5,
        ));
        insert_player(Player::parse(
            player_name::R_LF.to_string(),
            team_name::R.to_string(),
            2,
            5,
        ));

        // Blue team
        insert_player(Player::parse(
            player_name::B_GK.to_string(),
            team_name::B.to_string(),
            1,
            7,
        ));
        insert_player(Player::parse(
            player_name::B_LD.to_string(),
            team_name::B.to_string(),
            0,
            6,
        ));
        insert_player(Player::parse(
            player_name::B_RD.to_string(),
            team_name::B.to_string(),
            2,
            6,
        ));
        insert_player(Player::parse(
            player_name::B_MF.to_string(),
            team_name::B.to_string(),
            1,
            4,
        ));
        insert_player(Player::parse(
            player_name::B_LF.to_string(),
            team_name::B.to_string(),
            0,
            2,
        ));
        insert_player(Player::parse(
            player_name::B_RF.to_string(),
            team_name::B.to_string(),
            2,
            2,
        ));

        let mut goals = HashMap::new();
        goals.insert(team_name::R.to_string(), 0);
        goals.insert(team_name::B.to_string(), 0);

        let events: Vec<Event> = vec![Game::create_event(
            String::from("0:00"),
            String::from("The game starts. The Situation is B 00 - R 00. B-GK has the ball."),
        )];

        Game {
            players,
            goals,
            name_of_player_with_ball: player_name::B_GK.to_string(),
            events,
            round: 1,
            total_rounds: 10,
        }
    }

    pub fn which_team_has_ball(&self) -> Option<&String> {
        match self.players.get(&self.name_of_player_with_ball) {
            Some(p) => Some(&p.team),
            _ => None,
        }
    }

    pub fn pass_ball_to(&mut self, player: &Option<String>) -> Result<(), String> {
        let receiving_player_name = match player {
            Some(p) => p.to_string(),
            _ => return Err("Player argument not found".to_string()),
        };

        let passing_player = match self.players.get(&self.name_of_player_with_ball) {
            Some(p) => p,
            _ => return Err("Passing player not found".to_string()),
        };

        let receiving_player = match self.players.get(&receiving_player_name) {
            Some(p) => p,
            _ => return Err("Pass receiving player not found".to_string()),
        };

        let mut interrupting_player: Option<&Player> = None;
        for player in self.players.values() {
            if passing_player.team == player.team {
                continue;
            }

            let distance_to_passing_player =
                Game::distance_between_players(&passing_player, &player);
            let distance_to_receiving_player =
                Game::distance_between_players(&receiving_player, &player);

            if Game::is_pass_interrupted(distance_to_passing_player)
                || Game::is_pass_interrupted(distance_to_receiving_player)
            {
                interrupting_player = Some(player);
            }
        }

        match interrupting_player {
            Some(p) => {
                self.events.push(Game::create_event(
                    format!("{}:00", self.round),
                    format!(
                        "{} tries to pass ball to {} but {} interrupts the pass.",
                        passing_player.name, receiving_player.name, p.name
                    ),
                ));
                self.name_of_player_with_ball = p.name.clone();
            }
            None => {
                self.events.push(Game::create_event(
                    format!("{}:00", self.round),
                    format!(
                        "{} passes the ball to {}.",
                        self.name_of_player_with_ball, receiving_player.name
                    ),
                ));
                self.name_of_player_with_ball = receiving_player.name.clone();
            }
        }

        self.round += 1;
        Ok(())
    }

    pub fn shoot_ball_to(&mut self, team_goal: &Option<String>) -> Result<(), String> {
        let team_name = match team_goal {
            Some(t) => t.to_string(),
            _ => return Err("Team argument not found".to_string()),
        };

        let shooting_player = match self.players.get(&self.name_of_player_with_ball) {
            Some(p) => p,
            _ => return Err("Shooting player not found".to_string()),
        };

        let goalkeeper_name = match team_name.as_str() {
            team_name::R => player_name::R_GK,
            team_name::B => player_name::B_GK,
            _ => return Err("Team name not valid".to_string()),
        };

        let goalkeeper = match self.players.get(goalkeeper_name) {
            Some(gk) => gk,
            _ => return Err("Goalkeeper not found".to_string()),
        };

        if Game::does_shot_get_to_goal(Game::distance_between_players(
            &shooting_player,
            &goalkeeper,
        )) {
            match team_name.as_str() {
                team_name::R => *self.goals.get_mut(team_name::B).unwrap() += 1,
                _ => *self.goals.get_mut(team_name::R).unwrap() += 1,
            }

            self.events.push(Game::create_event(
                format!("{}:00", self.round),
                format!(
                    "{} shoots and scores! The Situation is B {} - R {}.",
                    self.name_of_player_with_ball,
                    self.goals[team_name::B],
                    self.goals[team_name::R]
                ),
            ));
        } else {
            self.events.push(Game::create_event(
                format!("{}:00", self.round),
                format!(
                    "{} shoots but fails to score. The Situation is still B {} - R {}.",
                    self.name_of_player_with_ball,
                    self.goals[team_name::B],
                    self.goals[team_name::R]
                ),
            ));
        }

        match team_name.as_str() {
            team_name::R => self.name_of_player_with_ball = player_name::R_GK.to_string(),
            _ => self.name_of_player_with_ball = player_name::B_GK.to_string(),
        }

        self.round += 1;
        Ok(())
    }

    pub fn has_ended(&mut self) -> bool {
        if self.round <= self.total_rounds {
            return false;
        }

        self.events.push(Game::create_event(
            format!("{}:00", self.round - 1),
            format!(
                "The Game has ended. The final score is B {} - R {}.",
                self.goals[team_name::B],
                self.goals[team_name::R]
            ),
        ));
        true
    }
}

impl Game {
    fn create_event(timestamp: String, description: String) -> Event {
        Event {
            timestamp,
            description,
        }
    }

    fn distance_between_players(p1: &Player, p2: &Player) -> u8 {
        let player_diff_on_x = p1.pos_x_axis as i8 - p2.pos_x_axis as i8;
        let player_diff_on_y = p1.pos_y_axis as i8 - p2.pos_y_axis as i8;
        (player_diff_on_x.abs() + player_diff_on_y.abs()) as u8
    }

    fn is_pass_interrupted(distance_to_target: u8) -> bool {
        if distance_to_target < 2 {
            return false;
        }

        let r: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let interruption_probability = 0.20;
        if distance_to_target == 2 && r <= interruption_probability {
            return true;
        }
        r < f32::max(
            interruption_probability - (0.1 * distance_to_target as f32),
            0.0,
        )
    }

    fn does_shot_get_to_goal(distance_to_goal: u8) -> bool {
        if distance_to_goal == 0 {
            return true;
        }

        let r: f32 = rand::thread_rng().gen_range(0.0..1.0);
        let probability = f32::min(0.366 * (distance_to_goal as f32 - 1.0), 0.95);
        probability < r
    }
}
