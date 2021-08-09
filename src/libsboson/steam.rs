// use serde::Deserialize;
// use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

// #[derive(Debug)]
// enum LibraryFoldersValues {
//     String,
//     HashMap<String, String>,
// }

// #[derive(Deserialize, Debug)]
// struct libraryfolders {
//
// };

pub fn get_steamapps_dirs() -> Option<Vec<PathBuf>> {
    // parse steam config
    let steam_home_dir: PathBuf = match find_steam_home_dir() {
        Some(path) => path,
        None => panic!("Unable to find Steam home directory. Is Steam installed?"),
    };

    let mut steam_library_folders_config_path: PathBuf = steam_home_dir.clone();
    steam_library_folders_config_path.push("config/libraryfolders.vdf");

    let mut steam_library_folders_config_file =
        File::open(steam_library_folders_config_path).ok()?;

    let mut steam_library_folders_config_data = String::new();
    steam_library_folders_config_file
        .read_to_string(&mut steam_library_folders_config_data)
        .ok()?;

    let mut steam_home_steamapps_dir = steam_home_dir.clone();
    steam_home_steamapps_dir.push("steamapps");

    let library_folders: Vec<PathBuf> =
        vec![PathBuf::from(steam_home_steamapps_dir), PathBuf::from("/")];

    return Some(library_folders);
}

pub fn find_steam_home_dir() -> Option<PathBuf> {
    macro_rules! check {
        ($path:expr, $suffix:expr) => {
            let candidate = Path::new($path).join($suffix);
            if candidate.is_dir() {
                return Some(candidate);
            }
        };
    }

    if let Some(xdg_data_home) = env::var_os("HOME") {
        check!(&xdg_data_home, "Steam");
    }

    if let Some(home) = env::var_os("HOME") {
        check!(&home, ".steam/steam");
    }

    if let Some(xdg_data_dirs) = env::var_os("XDG_DATA_DIRS") {
        for directory in xdg_data_dirs.as_bytes().split(|c| *c == b':') {
            check!(OsStr::from_bytes(directory), "Steam");
        }
    }

    None
}
