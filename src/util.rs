use std::{fs, path::PathBuf};

pub fn search_file(start_dir: &PathBuf, file_name: &str) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(start_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let metadata = fs::metadata(&path);

                if let Ok(metadata) = metadata {
                    if metadata.is_dir() {
                        if let Some(found_path) = search_file(&path, file_name) {
                            return Some(found_path);
                        }
                    } else if path.file_name().map(|f| f == file_name).unwrap_or(false) {
                        return Some(start_dir.to_path_buf());
                    }
                }
            }
        }
    }

    None
}

pub fn get_folder_name(path: &PathBuf) -> Option<String> {
    if path.is_dir() {
        path.file_name().and_then(|name| name.to_str().map(String::from))
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
pub fn get_pid_by_name(process_name: &str) -> u32 {
    use crate::find_pid_by_name;

    let wide_name: Vec<u16> = process_name.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        find_pid_by_name(wide_name.as_ptr())
    }
}

#[cfg(target_os = "windows")]
pub fn get_executable_path(pid: u32) -> Option<String> {
    use crate::get_process_path;

    let mut path: Vec<u16> = vec![0; 260];
    let size = path.len() as u32;
    let success = unsafe {
        get_process_path(pid, path.as_mut_ptr(), size)
    };
    if success {
        let path_string = String::from_utf16_lossy(&path);
        Some(path_string.trim_end_matches('\0').to_string())
    } else {
        None
    }
}


#[cfg(target_os = "windows")]
pub fn terminate_process_by_pid(pid: u32) -> bool {
    use crate::terminate_process;

    unsafe { terminate_process(pid) }
}

#[cfg(target_os = "windows")]
pub fn start_process_detached_(exe_path: &str) -> bool {
    use crate::start_process_detached;

    let wide_path: Vec<u16> = exe_path.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe { start_process_detached(wide_path.as_ptr()) } 
}