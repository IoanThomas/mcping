use clap::Parser;
use std::num::NonZeroU16;

const DEFAULT_SERVER_PORT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(25565) };

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Server IP address or hostname
    pub address: String,

    /// Server port
    #[clap(short = 'p', long = "port", default_value_t = DEFAULT_SERVER_PORT)]
    pub port: NonZeroU16,

    /// List the names of active players if the server provides them
    #[clap(short = 'l', long = "players")]
    pub list_players: bool,

    /// Print the encoded favicon (very long) if the server has one
    #[clap(short = 'f', long = "favicon")]
    pub show_favicon: bool,

    /// Print the raw JSON response (ignores other options)
    #[clap(short = 'j', long = "json")]
    pub json: bool,
}
