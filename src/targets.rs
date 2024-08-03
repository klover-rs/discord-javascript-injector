use std::path::PathBuf;

use crate::{constants::{DISCORD_VARIANTS, LOCAL_APP_DATA}, util::get_folder_name};

pub fn get_discord_path() -> Vec<PathBuf> {
    let mut discord_paths = Vec::new();
    let mut base_path = dirs::home_dir().unwrap();
    base_path.push(LOCAL_APP_DATA);
    
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