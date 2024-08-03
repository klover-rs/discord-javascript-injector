# discord-javascript-injector
This is a rust library which allows you to inject javascript into the discord desktop client!

With this project you can inject any kind of javascript you want into the discord client, the javascript code is attached to the renderer process, so you have access to the DOM etc

example usage
```rs
use inject::inject;
use eject::eject;
use check_installation::check_installed_clients;

fn main() {
    let installed_clients = check_installed_clients().unwrap();

    for client in installed_clients {
        println!("clinet: {:?}", client);

        if !client.injected {
            inject(&client.basename, "console.log('hello world!');").unwrap();
        } else {
            println!("already injected");
            eject(&client.basename).unwrap();
        }
    }

}
```
