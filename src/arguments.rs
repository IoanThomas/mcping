use clap::Parser;

const ADDRESS_HELP_TEXT: &str = "Server IP address or hostname";
const PORT_HELP_TEXT: &str = "Server port";
const LIST_PLAYERS_HELP_TEXT: &str =
    "List the names of active players, if the server provides them";

const DEFAULT_SERVER_PORT: u16 = 25565;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    #[clap(help = ADDRESS_HELP_TEXT)]
    pub address: String,
    #[clap(short, long, default_value_t = DEFAULT_SERVER_PORT, help = PORT_HELP_TEXT)]
    pub port: u16,
    #[clap(long, help = LIST_PLAYERS_HELP_TEXT)]
    pub list_players: bool,
}
