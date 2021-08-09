use crate::libsboson::steam::get_steamapps_dirs;
use std::env;
use std::result::Result;

pub fn get_game_name() -> Result<String, String> {
    let steam_app_id = env::var("SteamAppId").unwrap();
    // parse the main config vdf and get the main steam directory path
    let steamapps_dirs = get_steamapps_dirs().unwrap_or_default();

    // println!("{}", String::from(steamapps_dirs[0].as_os_str()));
    // look for a vdf file within steamapps in the main steam directory path that has the ID
    // parse the steamapp vdf file and return the name field.
    Ok(String::from(""))
}
