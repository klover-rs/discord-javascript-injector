mod asar;
mod targets;
mod util;
pub mod check_installation;
pub mod inject;
mod constants;
pub mod eject;

extern "C" {
    pub fn find_pid_by_name(process_name: *const u16) -> u32;
    pub fn get_process_path(pid: u32, path: *mut u16, size: u32) -> bool;
    pub fn start_process_detached(exe_path: *const u16) -> bool;
    pub fn terminate_process(pid: u32) -> bool;
}


#[cfg(test)]
mod tests {
    use check_installation::check_installed_clients;

    use super::*;

    #[test]
    fn check_installed_clients_test() {
        let installed_clients = check_installed_clients().unwrap();


        for client in installed_clients {
            println!("client: {:?}", client);
        }
    }
}