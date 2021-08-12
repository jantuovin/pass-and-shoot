use crate::computer_player;
use crate::game_logic;
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
    let mut game = game_logic::Game::new();

    ui::print_start_screen();
    ui::print_help();
    ui::print_field_and_latest_events(&game);

    loop {
        let action = match game.which_team_has_ball() {
            Some(t) => match t.as_str() {
                game_logic::team_name::B => Action::parse(&ui::read_input()),
                _ => Action::parse(&computer_player::read_input(&game)),
            },
            None => Action {
                command: String::from(allowed_command::HELP),
                argument: None,
            },
        };

        match action.command.as_str() {
            allowed_command::NEW => {
                game = game_logic::Game::new();
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
