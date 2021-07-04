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
        pub const B_RD: &str = "B_RF";
        pub const B_MF: &str = "B_MF";
        pub const B_LF: &str = "B_LF";
        pub const B_RF: &str = "B_RF";
    }

    mod team_name {
        pub const R: &str = "R";
        pub const B: &str = "B";
    }

    #[derive(Debug)]
    pub struct PlayerInfo {
        // TODO
    }

    #[derive(Debug)]
    pub struct Event {
        pub timestamp: String,
        pub description: String,
    }

    #[derive(Debug)]
    pub struct Game {
        pub players: HashMap<String, PlayerInfo>,
        pub goals: HashMap<String, u8>,
        pub player_with_ball: String,
        pub events: Vec<Event>,
        pub round: u8,
    }

    impl Game {
        pub fn new() -> Game {
            let mut players = HashMap::new();
            // Red team
            players.insert(player_name::R_GK.to_string(), PlayerInfo {});
            players.insert(player_name::R_LD.to_string(), PlayerInfo {});
            players.insert(player_name::R_RD.to_string(), PlayerInfo {});
            players.insert(player_name::R_MF.to_string(), PlayerInfo {});
            players.insert(player_name::R_LF.to_string(), PlayerInfo {});
            players.insert(player_name::R_RF.to_string(), PlayerInfo {});

            // Blue team
            players.insert(player_name::B_GK.to_string(), PlayerInfo {});
            players.insert(player_name::B_LD.to_string(), PlayerInfo {});
            players.insert(player_name::B_RD.to_string(), PlayerInfo {});
            players.insert(player_name::B_MF.to_string(), PlayerInfo {});
            players.insert(player_name::B_LF.to_string(), PlayerInfo {});
            players.insert(player_name::B_RF.to_string(), PlayerInfo {});

            let mut goals = HashMap::new();
            goals.insert(team_name::R.to_string(), 0);
            goals.insert(team_name::B.to_string(), 0);

            let events: Vec<Event> = vec![Event {
                timestamp: String::from("0:00"),
                description: String::from(
                    "The game starts. The Situation is B 00 - R 00. B-GK has the ball.",
                ),
            }];

            Game {
                players,
                goals,
                player_with_ball: player_name::B_GK.to_string(),
                events,
                round: 1,
            }
        }

        pub fn pass_ball_to(&mut self, player: &Option<String>) -> Result<(), String> {
            let player_str;
            match player {
                Some(p) => player_str = p.to_string(),
                _ => player_str = "".to_string(),
            }

            let player_exits = self.players.keys().any(|k| k == &player_str);
            if !player_exits {
                return Err("Player not found".to_string());
            }

            self.events.push(Event {
                timestamp: format!("{}:00", self.round),
                description: format!(
                    "{} passes the ball to {}.",
                    self.player_with_ball, player_str
                ),
            });

            self.player_with_ball = player_str;
            self.round += 1;
            Ok(())
        }

        pub fn shoot_ball_to(&mut self, teams_goal: &Option<String>) -> Result<(), String> {
            let team;
            match teams_goal {
                Some(p) => team = p.to_string(),
                _ => team = "".to_string(),
            }

            let team_exists = self.goals.keys().any(|k| k == &team);
            if !team_exists {
                return Err("Team not found".to_string());
            }

            if team == team_name::R {
                *self.goals.get_mut(team_name::B).unwrap() += 1;
            } else {
                *self.goals.get_mut(team_name::R).unwrap() += 1;
            }

            self.events.push(Event {
                timestamp: format!("{}:00", self.round),
                description: format!(
                    "{} scores! The Situation is B {} - R {}.",
                    self.player_with_ball,
                    self.goals[team_name::B],
                    self.goals[team_name::R]
                ),
            });

            match team.as_str() {
                team_name::R => {
                    self.player_with_ball = player_name::R_GK.to_string();
                }
                _ => {
                    self.player_with_ball = player_name::B_GK.to_string();
                }
            }
            self.round += 1;
            Ok(())
        }

        pub fn has_ended(&mut self) -> bool {
            if self.round <= 10 {
                return false;
            }

            self.events.push(Event {
                timestamp: format!("{}:00", self.round - 1),
                description: format!(
                    "The Game has ended. The final score is B {} - R {}.",
                    self.goals[team_name::B],
                    self.goals[team_name::R]
                ),
            });

            true
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
            let postfix = match player == &game.player_with_ball {
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
