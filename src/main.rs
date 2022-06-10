use clap::Parser;

const DEFAULT_SERVER_PORT: u16 = 25565;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(help = "Server IP address or hostname")]
    pub address: String,
    #[clap(short, long, default_value_t = DEFAULT_SERVER_PORT, help = "Server port")]
    pub port: u16,
    #[clap(
        long,
        help = "List the names of active players, if the server provides them"
    )]
    pub list_players: bool,
}

fn main() {
    let args = Args::parse();

    let response = match mcping::get_server_response(args.address, args.port) {
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

    if args.list_players {
        if let Some(players) = players {
            players
                .into_iter()
                .map(|player| player.name)
                .for_each(|name| println!("  {}", name));
        }
    }
}
