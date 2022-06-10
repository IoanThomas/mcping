[![Continuous Integration](https://github.com/IoanThomas/mcping/actions/workflows/ci.yml/badge.svg)](https://github.com/IoanThomas/mcping/actions/workflows/ci.yml)

# mcping
A simple tool to acquire Minecraft server information using the [Server List Ping interface](https://wiki.vg/Server_List_Ping).

`mcping` is a command-line tool that allows you to query any Minecraft Java Edition server for the basic information used in the game's server list.
This includes the server description, version, online players, and favicon.
You can configure the application using various command-line arguments.
Run `mcping --help` for full details.

## Building
Simply download the repository and execute `cargo build` in the project directory.
A binary should be created in `target/debug`.

## License
This project is licensed under the GNU AGPLv3 License - see the [LICENSE](LICENSE) file for details.
