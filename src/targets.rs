use std::path::PathBuf;

#[cfg(target_os = "linux")]
use crate::constants::CONFIG_FOLDER;
#[cfg(target_os = "windows")]
use crate::constants::LOCAL_APP_DATA;

use crate::{constants::DISCORD_VARIANTS, util::get_folder_name};

pub fn get_discord_path() -> Vec<PathBuf> {
    let mut discord_paths = Vec::new();
    let mut base_path = dirs::home_dir().unwrap();

    #[cfg(target_os = "windows")]
    base_path.push(LOCAL_APP_DATA);
    #[cfg(target_os = "linux")]
    base_path.push(CONFIG_FOLDER);
    
    for variant in &DISCORD_VARIANTS {
        let mut discord_path = base_path.clone();
        discord_path.push(variant);
        if discord_path.exists() {
            discord_paths.push(discord_path);
        }
    }

    discord_paths

}

pub fn find_target_client_path(which_discord: &str, targets: Vec<PathBuf>) -> Option<PathBuf> {
    for target in targets {
        if let Some(discord_folder_name) = get_folder_name(&target).map(|name| name.to_lowercase()) {
            if which_discord.to_lowercase() == discord_folder_name {
                return Some(target);
            }
        }
    }
    None
}