fn main() {
    crate::game_play::run();
}

mod core {
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
}

mod game_play {
    use crate::computer_player;
    use crate::core;
    use crate::ui;

    pub mod allowed_command {
        pub const NEW: &str = "new";
        pub const PASS: &str = "pass";
        pub const SHOOT: &str = "shoot";
        pub const FIELD: &str = "field";
        pub const INFO: &str = "info";
        pub const EVENTS: &str = "events";
        pub const QUIT: &str = "quit";
        pub const HELP: &str = "help";
    }

    #[derive(Debug)]
    struct Action {
        pub command: String,
        pub argument: Option<String>,
    }

    impl Action {
        fn parse(input: &String) -> Action {
            let input_split = input.split(" ");
            let parts: Vec<&str> = input_split.collect();

            if parts.len() == 0 {
                return Action {
                    command: String::from(allowed_command::HELP),
                    argument: None,
                };
            }

            let command = String::from(parts[0].trim());
            let argument = match parts.len() {
                2 => Some(String::from(parts[1].trim())),
                _ => None,
            };
            Action { command, argument }
        }
    }

    pub fn run() {
        let mut game = core::Game::new();

        ui::print_start_screen();
        ui::print_help();
        ui::print_field_and_latest_events(&game);

        loop {
            let action = match game.which_team_has_ball() {
                Some(t) => match t.as_str() {
                    core::team_name::B => Action::parse(&ui::read_input()),
                    _ => Action::parse(&computer_player::read_input(&game)),
                },
                None => Action {
                    command: String::from(allowed_command::HELP),
                    argument: None,
                },
            };

            match action.command.as_str() {
                allowed_command::NEW => {
                    game = core::Game::new();
                    ui::print_field_and_latest_events(&game);
                }
                allowed_command::PASS => match game.pass_ball_to(&action.argument) {
                    Ok(()) => {
                        ui::print_field_and_latest_events(&game);
                    }
                    _ => ui::print_help(),
                },
                allowed_command::SHOOT => match game.shoot_ball_to(&action.argument) {
                    Ok(()) => {
                        ui::print_field_and_latest_events(&game);
                    }
                    _ => ui::print_help(),
                },
                allowed_command::FIELD => ui::print_field(&game),
                allowed_command::INFO => ui::print_player_info(),
                allowed_command::EVENTS => ui::print_events(&game.events, &action.argument),
                allowed_command::QUIT => break,
                _ => ui::print_help(),
            }

            if game.has_ended() {
                ui::print_events(&game.events, &Some(String::from("5")));
                break;
            }
        }
    }
}

mod ui {
    use crate::core;
    use std::cmp;
    use std::io::{self, Write};

    pub fn print_start_screen() {
        println!("Welcome to play Pass and Shoot!");
    }

    pub fn print_help() {
        println!(
            "\nCommands:\n\
        \thelp: print this help\n\
        \tnew: start a new game\n\
        \tpass [PLAYER]: try pass the ball to player\n\
        \tshoot: [GOAL] shoot towards goal\n\
        \tfield: show field\n\
        \tinfo [PLAYER]: show player info\n\
        \tevents [ROWS]: show events\n\
        \tquit: quit the program"
        );
    }

    pub fn print_field(game: &core::Game) {
        let parse_player_text = |player: &str| -> String {
            let postfix = match player == &game.name_of_player_with_ball {
                true => "*",
                _ => " ",
            };
            let mut text = String::from(player);
            text.push_str(postfix);
            text
        };

        println!(
            "\n\
        ___________        ___________\n\
       |             {}            |\n\
       |                              |\n\
       |       {}       {}      |\n\
       |                              |\n\
       |     {}           {}    |\n\
       |                              |\n\
       |             {}            |\n\
       |                              |\n\
       |------------------------------|\n\
       |                              |\n\
       |             {}            |\n\
       |                              |\n\
       |     {}            {}   |\n\
       |                              |\n\
       |       {}       {}      |\n\
       |                              |\n\
       |___________  {} ___________|",
            parse_player_text(core::player_name::R_GK),
            parse_player_text(core::player_name::R_RD),
            parse_player_text(core::player_name::R_LD),
            parse_player_text(core::player_name::B_LF),
            parse_player_text(core::player_name::B_RF),
            parse_player_text(core::player_name::R_MF),
            parse_player_text(core::player_name::B_MF),
            parse_player_text(core::player_name::R_RF),
            parse_player_text(core::player_name::R_LF),
            parse_player_text(core::player_name::B_LD),
            parse_player_text(core::player_name::B_RD),
            parse_player_text(core::player_name::B_GK),
        );
    }

    pub fn print_player_info() {
        println!("\nNot implemented");
    }

    pub fn print_events(events: &Vec<core::Event>, how_many: &Option<String>) {
        let n_str = match how_many {
            Some(n) => n,
            _ => "",
        };

        let printed_events = match n_str.parse::<usize>() {
            Ok(n) => &events[events.len() - (cmp::min(n, events.len()))..],
            _ => &events[..],
        };

        println!("\n");
        for event in printed_events {
            println!("[{}] {}", event.timestamp, event.description);
        }
    }

    pub fn print_field_and_latest_events(game: &core::Game) {
        print_events(&game.events, &Some("5".to_string()));
        print_field(game);
    }

    pub fn read_input() -> String {
        print!("\nEnter command: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input
    }
}

mod computer_player {
    use crate::core;
    use crate::game_play;
    use rand::Rng;
    use std::{thread, time};

    pub fn read_input(game: &core::Game) -> String {
        thread::sleep(time::Duration::from_millis(1000));

        let player_with_ball = match game.players.get(&game.name_of_player_with_ball) {
            Some(p) => p,
            _ => return String::from(game_play::allowed_command::QUIT),
        };

        let possible_team_mates_to_pass: Vec<&core::Player> = game
            .players
            .values()
            .filter(|p| p.team == player_with_ball.team)
            .filter(|p| match p.team.as_str() {
                core::team_name::R => player_with_ball.pos_y_axis < p.pos_y_axis,
                _ => p.pos_y_axis < player_with_ball.pos_y_axis,
            })
            .collect();

        if possible_team_mates_to_pass.len() > 0 && (game.round / game.total_rounds) as f32 <= 0.90
        {
            let i = rand::thread_rng().gen_range(0..(possible_team_mates_to_pass.len() - 1));
            return format!(
                "{} {}",
                game_play::allowed_command::PASS,
                possible_team_mates_to_pass[i].name
            );
        }

        match player_with_ball.team.as_str() {
            core::team_name::R => format!(
                "{} {}",
                game_play::allowed_command::SHOOT,
                core::team_name::B
            ),
            _ => format!(
                "{} {}",
                game_play::allowed_command::SHOOT,
                core::team_name::R
            ),
        }
    }
}
