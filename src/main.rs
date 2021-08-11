fn main() {
    crate::game_play::run();
}

mod game_play {
    use crate::core;
    use crate::ui;

    #[derive(Debug)]
    enum Command {
        New,
        Pass,
        Shoot,
        Field,
        Info,
        Events,
        Quit,
        Help,
    }

    #[derive(Debug)]
    struct UserAction {
        pub command: Command,
        pub argument: Option<String>,
    }

    impl UserAction {
        fn parse(input: &String) -> UserAction {
            let input_split = input.split(" ");
            let parts: Vec<&str> = input_split.collect();

            if parts.len() == 0 {
                return UserAction {
                    command: Command::Help,
                    argument: None,
                };
            }

            let command = match parts[0].trim() {
                "new" => Command::New,
                "pass" => Command::Pass,
                "shoot" => Command::Shoot,
                "field" => Command::Field,
                "info" => Command::Info,
                "events" => Command::Events,
                "quit" => Command::Quit,
                _ => Command::Help,
            };

            let argument = match parts.len() {
                2 => Some(String::from(parts[1].trim())),
                _ => None,
            };

            UserAction { command, argument }
        }
    }

    pub fn run() {
        let mut game = core::Game::new();

        ui::print_start_screen();
        ui::print_help();
        ui::print_field_and_latest_events(&game);

        loop {
            let input = ui::read_user_input();
            let action = UserAction::parse(&input);

            match action.command {
                Command::New => {
                    game = core::Game::new();
                    ui::print_field_and_latest_events(&game);
                }
                Command::Pass => match game.pass_ball_to(&action.argument) {
                    Ok(()) => {
                        ui::print_field_and_latest_events(&game);
                    }
                    _ => ui::print_help(),
                },
                Command::Shoot => match game.shoot_ball_to(&action.argument) {
                    Ok(()) => {
                        ui::print_field_and_latest_events(&game);
                    }
                    _ => ui::print_help(),
                },
                Command::Field => ui::print_field(&game),
                Command::Info => ui::print_player_info(),
                Command::Events => ui::print_events(&game.events, &action.argument),
                Command::Quit => break,
                _ => ui::print_help(),
            }

            if game.has_ended() {
                ui::print_events(&game.events, &Some("5".to_string()));
                break;
            }
        }
    }
}

mod core {
    use rand::Rng;
    use std::collections::HashMap;

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

    mod team_name {
        pub const R: &str = "R";
        pub const B: &str = "B";
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
    }

    impl Game {
        pub fn new() -> Game {
            let mut players = HashMap::new();
            // Red team
            players.insert(
                player_name::R_GK.to_string(),
                Player::parse(
                    player_name::R_GK.to_string(),
                    team_name::R.to_string(),
                    1,
                    0,
                ),
            );
            players.insert(
                player_name::R_RD.to_string(),
                Player::parse(
                    player_name::R_RD.to_string(),
                    team_name::R.to_string(),
                    0,
                    1,
                ),
            );
            players.insert(
                player_name::R_LD.to_string(),
                Player::parse(
                    player_name::R_RD.to_string(),
                    team_name::R.to_string(),
                    2,
                    1,
                ),
            );
            players.insert(
                player_name::R_MF.to_string(),
                Player::parse(
                    player_name::R_RD.to_string(),
                    team_name::R.to_string(),
                    1,
                    3,
                ),
            );
            players.insert(
                player_name::R_RF.to_string(),
                Player::parse(
                    player_name::R_RD.to_string(),
                    team_name::R.to_string(),
                    0,
                    5,
                ),
            );
            players.insert(
                player_name::R_LF.to_string(),
                Player::parse(
                    player_name::R_RD.to_string(),
                    team_name::R.to_string(),
                    2,
                    5,
                ),
            );

            // Blue team
            players.insert(
                player_name::B_GK.to_string(),
                Player::parse(
                    player_name::B_GK.to_string(),
                    team_name::B.to_string(),
                    1,
                    7,
                ),
            );
            players.insert(
                player_name::B_LD.to_string(),
                Player::parse(
                    player_name::B_LD.to_string(),
                    team_name::B.to_string(),
                    0,
                    6,
                ),
            );
            players.insert(
                player_name::B_RD.to_string(),
                Player::parse(
                    player_name::B_RD.to_string(),
                    team_name::B.to_string(),
                    2,
                    6,
                ),
            );
            players.insert(
                player_name::B_MF.to_string(),
                Player::parse(
                    player_name::B_MF.to_string(),
                    team_name::B.to_string(),
                    1,
                    4,
                ),
            );
            players.insert(
                player_name::B_LF.to_string(),
                Player::parse(
                    player_name::B_LF.to_string(),
                    team_name::B.to_string(),
                    0,
                    2,
                ),
            );
            players.insert(
                player_name::B_RF.to_string(),
                Player::parse(
                    player_name::B_RF.to_string(),
                    team_name::B.to_string(),
                    2,
                    2,
                ),
            );

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
                None => {
                    self.events.push(Game::create_event(
                        format!("{}:00", self.round),
                        format!(
                            "{} passes the ball to {}.",
                            self.name_of_player_with_ball, receiving_player_name
                        ),
                    ));
                    self.name_of_player_with_ball = receiving_player_name;
                }
                Some(p) => {
                    self.events.push(Game::create_event(
                        format!("{}:00", self.round),
                        format!("{} interrupts the pass.", p.name),
                    ));
                    self.name_of_player_with_ball = p.name.clone();
                }
            }

            self.round += 1;
            Ok(())
        }

        pub fn shoot_ball_to(&mut self, teams_goal: &Option<String>) -> Result<(), String> {
            let team_name = match teams_goal {
                Some(p) => p.to_string(),
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

            let team_goalkeeper = match self.players.get(goalkeeper_name) {
                Some(gk) => gk,
                _ => return Err("Goalkeeper not found".to_string()),
            };

            if Game::does_shot_get_to_goal(Game::distance_between_players(
                &shooting_player,
                &team_goalkeeper,
            )) {
                match team_name.as_str() {
                    team_name::R => *self.goals.get_mut(team_name::B).unwrap() += 1,
                    _ => *self.goals.get_mut(team_name::R).unwrap() += 1,
                }

                self.events.push(Game::create_event(
                    format!("{}:00", self.round),
                    format!(
                        "{} scores! The Situation is B {} - R {}.",
                        self.name_of_player_with_ball,
                        self.goals[team_name::B],
                        self.goals[team_name::R]
                    ),
                ));
            } else {
                self.events.push(Game::create_event(
                    format!("{}:00", self.round),
                    format!(
                        "{} fails to score. The Situation is still B {} - R {}.",
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
            if self.round <= 10 {
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

            let mut rng = rand::thread_rng();
            let r: f32 = rng.gen_range(0.0..1.0);
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

            let mut rng = rand::thread_rng();
            let r: f32 = rng.gen_range(0.0..1.0);
            let probability = f32::min(0.366 * (distance_to_goal as f32 - 1.0), 0.95);
            probability < r
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

        let red_gk = parse_player_text(core::player_name::R_GK);
        let red_rd = parse_player_text(core::player_name::R_RD);
        let red_ld = parse_player_text(core::player_name::R_LD);
        let blue_lf = parse_player_text(core::player_name::B_LF);
        let blue_rf = parse_player_text(core::player_name::B_RF);
        let red_mf = parse_player_text(core::player_name::R_MF);
        let blue_mf = parse_player_text(core::player_name::B_MF);
        let red_rf = parse_player_text(core::player_name::R_RF);
        let red_lf = parse_player_text(core::player_name::R_LF);
        let blue_ld = parse_player_text(core::player_name::B_LD);
        let blue_rd = parse_player_text(core::player_name::B_RD);
        let blue_gk = parse_player_text(core::player_name::B_GK);

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
            red_gk,
            red_rd,
            red_ld,
            blue_lf,
            blue_rf,
            red_mf,
            blue_mf,
            red_rf,
            red_lf,
            blue_ld,
            blue_rd,
            blue_gk
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
        print_field(game);
        print_events(&game.events, &Some("5".to_string()));
    }

    pub fn read_user_input() -> String {
        print!("\nEnter command: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input
    }
}
