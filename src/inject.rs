use std::{fs::{self, File}, io::{BufRead, BufReader, Write}, path::PathBuf};
use anyhow::{anyhow, Result};
use std::time::Duration;
use crate::{asar::*, constants::{CORE_ASAR_BACKUP_FILE, CORE_ASAR_FILE}, targets::{self, find_target_client_path}, util::{search_file, get_pid_by_name, get_executable_path, terminate_process_by_pid, start_process_detached_}};


use tokio_tungstenite::connect_async;

pub async fn inject_ws(which_discord: &str, javascript_to_inject: &str, ws_url: &str) -> Result<()> {

    let url = url::Url::parse(&ws_url)?;
    let (mut ws_stream, _) = connect_async(url).await?;

    let targets = targets::get_discord_path();
    
    let target_client = if let Some(target_client) = find_target_client_path(which_discord, targets) {
        target_client
    } else {
        return Err(anyhow!("couldnt find target client path"))
    };

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

    match search_file(&target_client, CORE_ASAR_FILE) {
        Some(path) => {
            
            if let Ok(metadata) = fs::metadata(path.join(CORE_ASAR_BACKUP_FILE)) {
                if metadata.is_file() {
                    return Err(anyhow!("cannot inject contents into an already injected file."))
                }
            }

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

            fs::copy(path.join(CORE_ASAR_FILE), path.join(CORE_ASAR_BACKUP_FILE))?;

            let dest_path = path.join("unpacked");

            extract_asar_ws(&path.join(CORE_ASAR_FILE), &dest_path, &mut ws_stream).await?;

            inject_javascript("inject.js", &javascript_to_inject, &dest_path.join("app"))?;

            pack_asar(&dest_path, &path.join(CORE_ASAR_FILE))?;

            fs::remove_dir_all(&dest_path)?;

            #[cfg(target_os = "windows")]
            {
                if let Some(exec_path) = executable_path {
                    let result = start_process_detached_(&exec_path);
                    if !result {
                        println!("failed to start process detached.")
                    }
                }
            }

        }
        None => {
            return Err(anyhow!("Couldnt find core.asar file"));
        }
    }

    Ok(())
}

pub fn inject(which_discord: &str, javascript_to_inject: &str) -> Result<()> {

    let targets = targets::get_discord_path();
    
    let target_client = if let Some(target_client) = find_target_client_path(which_discord, targets) {
        target_client
    } else {
        return Err(anyhow!("couldnt find target client path"))
    };


    match search_file(&target_client, CORE_ASAR_FILE) {
        Some(path) => {
            
            if let Ok(metadata) = fs::metadata(path.join(CORE_ASAR_BACKUP_FILE)) {
                if metadata.is_file() {
                    return Err(anyhow!("cannot inject contents into an already injected file."))
                }
            }

            fs::copy(path.join(CORE_ASAR_FILE), path.join(CORE_ASAR_BACKUP_FILE))?;

            let dest_path = path.join("unpacked");

            extract_asar(&path.join(CORE_ASAR_FILE), &dest_path)?;

            inject_javascript("inject.js", &javascript_to_inject, &dest_path.join("app"))?;

            pack_asar(&dest_path, &path.join(CORE_ASAR_FILE))?;

            fs::remove_dir_all(&dest_path)?;

        }
        None => {
            return Err(anyhow!("Couldnt find core.asar file"));
        }
    }

    Ok(())
}

fn inject_javascript(file_name: &str, javascript_content: &str, dest_path: &PathBuf) -> Result<()> {
    let mut full_path = dest_path.clone();

    full_path.push(file_name);

    let mut file = File::create(full_path)?;

    file.write_all(javascript_content.as_bytes())?;

    let main_screen = dest_path.join("mainScreen.js");
    let target_string = "  mainWindow = new _electron.BrowserWindow(mainWindowOptions);";

    let new_content = 
    r#"
      const path = require('path');
      const fs = require('fs');
      const js_inject_file = path.join(__dirname, 'inject.js');
      mainWindow.webContents.on('dom-ready', () => {
        setTimeout(() => {
          mainWindow.webContents.executeJavaScript(fs.readFileSync(js_inject_file) + "");
        }, 3000);
      });
    "#;
    
    inject_into_mainscreen(&main_screen, &target_string, &new_content)?;

    Ok(())
}

fn inject_into_mainscreen(main_screen_path: &PathBuf, target_string: &str, new_content: &str) -> Result<()> {
    let file = File::open(main_screen_path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut target_line_index: Option<usize> = None;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(target_string) {
            target_line_index = Some(index);
        }
        lines.push(line);
    }

    if let Some(index) = target_line_index {
        lines.insert(index + 1, new_content.to_string());
    } else {
        return Err(anyhow!("Target string not found in file"));
    };

    let mut file = File::create(main_screen_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}