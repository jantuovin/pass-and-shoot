use crate::game_logic;
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

pub fn print_field(game: &game_logic::Game) {
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
        parse_player_text(game_logic::player_name::R_GK),
        parse_player_text(game_logic::player_name::R_RD),
        parse_player_text(game_logic::player_name::R_LD),
        parse_player_text(game_logic::player_name::B_LF),
        parse_player_text(game_logic::player_name::B_RF),
        parse_player_text(game_logic::player_name::R_MF),
        parse_player_text(game_logic::player_name::B_MF),
        parse_player_text(game_logic::player_name::R_RF),
        parse_player_text(game_logic::player_name::R_LF),
        parse_player_text(game_logic::player_name::B_LD),
        parse_player_text(game_logic::player_name::B_RD),
        parse_player_text(game_logic::player_name::B_GK),
    );
}

pub fn print_player_info() {
    println!("\nNot implemented");
}

pub fn print_events(events: &Vec<game_logic::Event>, how_many: &Option<String>) {
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

pub fn print_field_and_latest_events(game: &game_logic::Game) {
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
