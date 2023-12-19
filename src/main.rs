use crate::arguments::Arguments;
use clap::Parser;
use mcping::response::Description;
use std::borrow::Cow;

mod arguments;

fn main() {
    let arguments = Arguments::parse();

    match arguments.json {
        true => json_response(arguments),
        false => regular_response(arguments),
    };
}

fn json_response(arguments: Arguments) {
    let response = match mcping::get_server_response_json(arguments.address, arguments.port) {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Error: {error}");
            return;
        }
    };

    println!("{response}");
}

fn regular_response(arguments: Arguments) {
    let response = match mcping::get_server_response(arguments.address, arguments.port) {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Error: {error}");
            return;
        }
    };

    let description = match response.description {
        Description::Text(text) => text,
        Description::Chat(chat) => chat.text,
    };
    let version_name = response.version.name;
    let version_protocol = response.version.protocol;
    let players_online = response.players.online;
    let players_max = response.players.max;
    let players = response.players.sample;
    let favicon = response.favicon;

    println!("Description: \"{description}\"");
    println!("Version: \"{version_name}\" ({version_protocol})");
    println!("Players: {players_online}/{players_max}");

    if arguments.list_players {
        if let Some(players) = players {
            players
                .into_iter()
                .map(|player| player.name)
                .for_each(|name| println!("\t{name}"));
        }
    }

    println!("{}", favicon_line(favicon, arguments.show_favicon));
}

fn favicon_line(favicon: Option<String>, show_favicon: bool) -> Cow<'static, str> {
    match (favicon, show_favicon) {
        (None, _) => "Has Favicon: No".into(),
        (Some(_), false) => "Has Favicon: Yes".into(),
        (Some(favicon), true) => format!("Favicon: {favicon}").into(),
    }
}
