use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

use crate::{constants::{CORE_ASAR_BACKUP_FILE, CORE_ASAR_FILE}, targets, util::{get_folder_name, search_file}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientInfo {
    pub path: String,
    pub basename: String,
    pub injected: bool,
}

pub fn check_installed_clients() -> Result<Vec<ClientInfo>> {
    let targets = targets::get_discord_path();

    let mut client_info = Vec::new();

    for folder_path in targets {
        let core_folder = match search_file(&folder_path, CORE_ASAR_FILE) {
            Some(path) => path,
            None => {
                println!("core.asar was not found");
                continue;
            }
        };

        let core_backup_path = core_folder.join(CORE_ASAR_BACKUP_FILE);
        let file_exists = if let Ok(metdata) = fs::metadata(&core_backup_path) {
            metdata.is_file()
        } else {
            false
        };

        let folder_name = get_folder_name(&folder_path);

        if let None = folder_name {
            return Err(anyhow!("not a folder"));
        }

        let client_details = ClientInfo {
            path: core_folder.to_string_lossy().into_owned(),
            basename: folder_name.unwrap(),
            injected: file_exists
        };

        client_info.push(client_details);

    }

    Ok(client_info)
}