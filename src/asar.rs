use std::{fs::{self, File}, path::PathBuf};
use asar::{AsarReader, AsarWriter, HashAlgorithm, Result as AsarResult};
use walkdir::WalkDir;

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

pub fn extract_asar(asar_file: &PathBuf, output: &PathBuf) -> AsarResult<String> {
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
    }

    println!("total files: {}", total_files);
    Ok(total_files.to_string())
}