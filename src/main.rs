use clap::Parser;

const DEFAULT_SERVER_PORT: u16 = 25565;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    pub address: String,
    #[clap(short, long)]
    pub port: Option<u16>,
}

fn main() {
    let args = Args::parse();

    let address = args.address;
    let port = args.port.unwrap_or(DEFAULT_SERVER_PORT);

    let response = match mcping::get_server_response(address, port) {
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

    println!("Description: {}", description);
    println!("Version: \"{}\" ({})", version_name, version_protocol);
    println!("Players: {}/{}", players_online, players_max);
}
