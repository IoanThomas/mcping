use crate::arguments::Arguments;
use clap::Parser;

mod arguments;

fn main() {
    let arguments = Arguments::parse();

    let response = match mcping::get_server_response(arguments.address, arguments.port) {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    let description = response.description.text;
    let version_name = response.version.name;
    let version_protocol = response.version.protocol;
    let players_online = response.players.online;
    let players_max = response.players.max;
    let players = response.players.sample;

    println!("Description: {}", description);
    println!("Version: \"{}\" ({})", version_name, version_protocol);
    println!("Players: {}/{}", players_online, players_max);

    if arguments.list_players {
        if let Some(players) = players {
            players
                .into_iter()
                .map(|player| player.name)
                .for_each(|name| println!("  {}", name));
        }
    }
}
