use std::fs;
use anyhow::{Result, anyhow};

use crate::{constants::{CORE_ASAR_BACKUP_FILE, CORE_ASAR_FILE}, targets::{self, find_target_client_path}, util::search_file}; 

pub fn eject(which_discord: &str) -> Result<()> {

    let targets = targets::get_discord_path();

    let target_client = if let Some(target_client) = find_target_client_path(which_discord, targets) {
        target_client
    } else {
        return Err(anyhow!("couldnt find target client path"))
    };

    match search_file(&target_client, CORE_ASAR_BACKUP_FILE) {
        Some(path) => {
            fs::remove_file(path.join(CORE_ASAR_FILE))?;
            fs::rename(path.join(CORE_ASAR_BACKUP_FILE), path.join(CORE_ASAR_FILE))?;
            Ok(())
        }
        None => {
            return Err(anyhow!("Couldnt find core.asar.backup"));
        }
    }

}