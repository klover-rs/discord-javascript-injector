use std::{fs, time::Duration};
use anyhow::{Result, anyhow};

use crate::{constants::{CORE_ASAR_BACKUP_FILE, CORE_ASAR_FILE}, targets::{self, find_target_client_path}, util::search_file}; 


#[cfg(target_os = "windows")]
use crate::util::{get_pid_by_name, get_executable_path, terminate_process_by_pid, start_process_detached_};


pub fn eject(which_discord: &str) -> Result<()> {

    #[cfg(target_os = "windows")]
    let mut pid: Option<u32> = None;

    #[cfg(target_os = "windows")]
    {
        let pid_ = get_pid_by_name(&format!("{}.exe", &which_discord));
        
        if pid_ != 0 {
            pid = Some(pid_);
        } else {
            println!("no process found with pid: {}", pid_);
        }
    }

    let targets = targets::get_discord_path();

    let target_client = if let Some(target_client) = find_target_client_path(which_discord, targets) {
        target_client
    } else {
        return Err(anyhow!("couldnt find target client path"))
    };

    match search_file(&target_client, CORE_ASAR_BACKUP_FILE) {
        Some(path) => {

            #[cfg(target_os = "windows")]
            let mut executable_path: Option<String> = None;

            #[cfg(target_os = "windows")]
            {
                if let Some(pid) = pid {
                    executable_path = get_executable_path(pid);
                    if !terminate_process_by_pid(pid) {
                        return Err(anyhow!("failed to terminate process."))
                    };
                    std::thread::sleep(Duration::from_secs(2)); // wait for 2 seconds so that the process can be killed
                }
            }

            fs::remove_file(path.join(CORE_ASAR_FILE))?;
            fs::rename(path.join(CORE_ASAR_BACKUP_FILE), path.join(CORE_ASAR_FILE))?;

            #[cfg(target_os = "windows")]
            {
                if let Some(exec_path) = executable_path {
                    let result = start_process_detached_(&exec_path);
                    if !result {
                        println!("failed to start process detached.")
                    }
                }
            }

            Ok(())
        }
        None => {
            return Err(anyhow!("Couldnt find core.asar.backup"));
        }
    }

   
}