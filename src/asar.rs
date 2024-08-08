use std::{fs::{self, File}, path::PathBuf};
use asar::{AsarReader, AsarWriter, HashAlgorithm, Result as AsarResult};
use walkdir::WalkDir;
use rayon::prelude::*;
use anyhow::Result as AnyResult;

pub fn pack_asar(path: &PathBuf, dest: &PathBuf) -> AnyResult<()> {
    let mut writer = AsarWriter::new_with_algorithm(HashAlgorithm::Sha256);

    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_content = fs::read(entry.path())?;

            let relative_path = entry.path().strip_prefix(path)?;
            let relative_path_str = relative_path.to_str().unwrap_or("");
            writer.write_file(relative_path_str, &file_content, false)?;
        }
    }

    let mut output_file = File::create(dest)?;
    let bytes_written = writer.finalize(&mut output_file)?;

    println!("wrote {} bytes to {:?}", bytes_written * 1000, dest);

    Ok(())
}

pub fn extract_asar(asar_file: &PathBuf, output: &PathBuf) -> AsarResult<()> {
    let asar_file = fs::read(asar_file)?;
    let asar = AsarReader::new(&asar_file, None)?;


    let total_files = asar.files().len() as f64;
    let counter = std::sync::atomic::AtomicUsize::new(0);


    asar.files().par_iter().try_for_each(|(path, file_info)| -> AsarResult<()> {
        let output_path = output.join(path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        
        let content = file_info.data();
        fs::write(&output_path, content)?;
        let current_count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;

        let progress_percentage = (current_count as f64 / total_files) * 100.0;
        println!("extracted: {} | {:.2}%", output_path.display(), progress_percentage);

        Ok(())
    })?;

    println!("total files: {}", total_files);

    Ok(())
}


use tokio_tungstenite::tungstenite::Message;
use futures_util::sink::SinkExt;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;


pub async fn pack_asar_ws(path: &PathBuf, dest: &PathBuf, ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) -> AnyResult<()> {
    let mut writer = AsarWriter::new_with_algorithm(HashAlgorithm::Sha256);

    
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_content = fs::read(entry.path())?;

            let relative_path = entry.path().strip_prefix(path)?;
            let relative_path_str = relative_path.to_str().unwrap_or("");
            writer.write_file(&relative_path_str, &file_content, false)?;

            let progress = format!("packing: {}", &relative_path_str);

            println!("{}", &progress);

            ws_stream.send(Message::Text(progress)).await.unwrap();
        }
    }

    let mut output_file = File::create(dest)?;
    let _bytes_written = writer.finalize(&mut output_file)?;

    Ok(())
}

pub async fn extract_asar_ws(asar_file: &PathBuf, output: &PathBuf, ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) -> AsarResult<String> {

    let asar_file = fs::read(asar_file)?;
    let asar = AsarReader::new(&asar_file, None)?;

    let total_files = asar.files().len() as f64;
    let mut counter = 0;

    
    for (path, file_info) in asar.files() {
        let output_path = output.join(path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = file_info.data();
        fs::write(&output_path, content)?;
        counter += 1;

        let progress_percentage = (counter as f64 / total_files) * 100.0;

        println!("extracted: {} | {:.2}%", output_path.display(), progress_percentage);

        let progress_message = format!("extracted: {} | {:.2}%", output_path.display(), progress_percentage);
        ws_stream.send(Message::Text(progress_message)).await.unwrap();
    }

    println!("total files: {}", total_files);
    Ok(total_files.to_string())
}