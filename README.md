# discord-javascript-injector
This is a rust library which allows you to inject javascript into the discord desktop client!

With this project you can inject any kind of javascript you want into the discord client, the javascript code is attached to the renderer process, so you have access to the DOM etc

~~Please note that only windows is yet developed, linux support is coming next~~
welp, that is now done!
if you have installed discord with a .deb file the folder containing the core.asar file should be in your .config folder

## Next goal
- ~~macOS support~~ i am happy to announce that macOS support has been added! (thank you for not letting me down my dear m1 macbook air)
- flatpak support for linux
- snap (for linux) support will be investigated in the future.

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
            inject(&client.basename, "console.log('hello world!');", false).unwrap();
        } else {
            println!("already injected");
            eject(&client.basename).unwrap();
        }
    }

}
```

## Another cool feature!
Well, since i expect that this project will be used in some gui applications, i decided to make an ws (websocket) feature, which allows you to connect the library to your own websocket server to receive messages about the progress of the injection.

to use the ws feature you can add this to your cargo.toml

```toml
[dependencies]
discord_injector-lib = { version = "x.x.x" features = ["ws"] }
```
please note that `x.x.x` is a place holder. version from 0.3.0 and above support this feature :).

but thats not it, typescript is also now officially supported. 

thanks to [swc](https://swc.rs/) its not possible to compile typescript to javascript, so enjoy type safety features with this library! 

**attention tho** 
in my tests i came to the conclusion, that enums in typescript are not being properly translated to javascript, this issue is none of my responsibility but rather swcs responsibility, so just avoid using enums, thanks :D

### Check it out on crates.io!
I also created a library on crates.io, here is the [link](https://crates.io/crates/discord_injector-lib) to it

### IMPORTANT
This project might break your client. in case of client breaks, please reinstall discord for your platform. 

