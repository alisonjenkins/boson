pub mod libsboson;

fn main() {
    let args = libsboson::arg_parsing::get_args().unwrap();
    let config = libsboson::config::get_config(args.value_of("config").unwrap_or(""));

    let game_name = match libsboson::steam_app::get_game_name() {
        Ok(name) => name,
        Err(err) => {
            panic!("{}", err);
        }
    };

    println!("{}", game_name);
}
