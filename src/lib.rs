mod asar;
mod targets;
mod util;
pub mod check_installation;
pub mod inject;
mod constants;
pub mod eject;

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