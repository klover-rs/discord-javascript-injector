use std::{fs::{self, File}, io::{BufRead, BufReader, Write}, path::PathBuf};
use anyhow::{anyhow, Result};
use std::time::Duration;
use crate::{asar::*, constants::{CORE_ASAR_BACKUP_FILE, CORE_ASAR_FILE}, targets::{self, find_target_client_path}, util::{search_file, get_pid_by_name, get_executable_path, terminate_process_by_pid, start_process_detached_}};
use futures_util::SinkExt;

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use swc_common::SourceMap;
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_transforms_typescript::strip_type;
use swc_ecma_visit::VisitMutWith;
use swc_common::FileName;

use std::rc::Rc;

fn gen_javascript(content: &str) -> String {
    let cm = SourceMap::new(swc_common::FilePathMapping::empty());

    let fm = cm.new_source_file(Rc::new(FileName::Custom("inline".into())), content.into());

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let mut module = parser
        .parse_module()
        .expect("failed to parse input as a module");


    let lcm: std::rc::Rc<SourceMap> = cm.into();

    module.visit_mut_with(&mut strip_type());

    let code = {
        let mut buf = Vec::new();
        {
            let mut emitter = Emitter {
                cfg: Default::default(),
                cm: lcm.clone(),
                comments: None,
                wr: JsWriter::new(lcm.clone(), "\n", &mut buf, None),
            };

            emitter.emit_module(&module).unwrap();
        }

        String::from_utf8_lossy(&buf).to_string()
    };

    code
}


pub async fn inject_ws(which_discord: &str, javascript_to_inject: &str, is_typescript: bool, ws_url: &str) -> Result<()> {

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

            let javascript_content = if is_typescript {
                gen_javascript(&javascript_to_inject)
            } else {
                javascript_to_inject.to_string()
            };

            inject_javascript("inject.js", &javascript_content, &dest_path.join("app"))?;

            pack_asar_ws(&dest_path, &path.join(CORE_ASAR_FILE), &mut ws_stream).await?;

            fs::remove_dir_all(&dest_path)?;

            #[cfg(target_os = "windows")]
            {
                if let Some(exec_path) = executable_path {
                    let result = start_process_detached_(&exec_path);
                    if !result {
                        ws_stream.send(Message::Text("failed to start process detached.".to_string())).await.unwrap();
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

pub fn inject(which_discord: &str, javascript_to_inject: &str, is_typescript: bool) -> Result<()> {

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

            let javascript_content = if is_typescript {
                gen_javascript(&javascript_to_inject)
            } else {
                javascript_to_inject.to_string()
            };

      

            inject_javascript("inject.js", &javascript_content, &dest_path.join("app"))?;

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


