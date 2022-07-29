use app_stopper::config::Config;
use app_stopper::sync::GistSync;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};
use std::env;
use std::env::consts::OS;
use std::io::{self, Write};
use std::process::{exit, Command};
use std::time::{Duration, Instant};
// Todo make window pop up on top of all other windows if help is requested and pause discord unrtill help is closed
// Todo make the interval for checking for Discord more often
// Todo stop useing unwrap() for everything
fn main() {
    let mut day = chrono::Local::today();
    // let mut time_left = 50;
    // let mut help_time = 5; // this defines time for help peroid
    let mut config = Config::read_config().unwrap();
    let gist_sync = GistSync::new();
    gist_sync
        .sync(config.gist.clone().unwrap(), config.apps.clone())
        .unwrap();
    env::args().for_each(|arg| {
        if arg == "--help" {
            println!("Aplication Stopper [--help] [--version] [--sync] [--help-time=<time>] [--time-left=<time>]");
            println!("This program is used to check if Discord is running and if it is, it will pause it.
It will also check if Discord is running and if it is not, it will resume it.");
            exit(0);
        } else if arg == "--version" {
            println!("application stopper v0.1.0");
            exit(0);
        } else if arg == "--sync" {
            print!("are you using a github gist (y/n): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                input.clear();
                print!("enter the github username: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                config.gist.as_mut().unwrap().github_user = input.trim().to_string();
                print!("please enter your github gist token (if needed): ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<String>() {
                    Ok(token) => {
                        if token.is_empty() {
                        } else {
                            config.gist.as_mut().unwrap().github_token = Some(token);
                        }}
                    Err(_) => {
                        config.gist.as_mut().unwrap().github_token = None;
                    }
                }
                input.clear();
                print!("please enter your github gist id: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                config.gist.as_mut().unwrap().gist_id = input.trim().to_string();
                input.clear();
                print!("please enter your github gist file name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                if !input.trim().ends_with(".toml") {
                    println!("the file name must end with .toml");
                    exit(0);
                }
                config.gist.as_mut().unwrap().gist_file_name = input.trim().to_string();
                input.clear();
                config.write_config().unwrap();
            } else {
                input.clear();
                print!("please enter theurl for the file you are using: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                match config.urls.as_mut() {
                    Some(urls) => {
                        urls.push(input.trim().to_string());
                    }
                    None => {
                        config.urls = Some(vec![input.trim().to_string()]);
                    }
                }
                input.clear();
                config.write_config().unwrap();
            }
        }
        else if arg.starts_with("--help-time") {
            config.set_help_time(arg.split('=').collect::<Vec<&str>>()[1].parse::<u64>().unwrap(), "Discord".to_string());
            gist_sync.sync(config.gist.clone().unwrap(), config.apps.clone()).unwrap();
        }
        else if arg.starts_with("--time-left") {
            config.set_time_left(arg.split('=').collect::<Vec<&str>>()[1].parse::<u64>().unwrap(), "Discord".to_string());
            gist_sync.sync(config.gist.clone().unwrap(), config.apps.clone()).unwrap();
        }
    });

    println!("{}", config.get_help_time("Discord".to_string()));
    loop {
        // check if its a new day
        if day != chrono::Local::today() {
            println!("New day!");
            day = chrono::Local::today();
            config.set_time_left(50, "Discord".to_string());
            gist_sync
                .sync(config.gist.clone().unwrap(), config.apps.clone())
                .unwrap();
        }
        println!("{}", config.get_time_left("Discord".to_string()));
        println!("type 'h' for help");
        let help = read_key(Duration::from_secs(120));
        match help {
            // Todo figureout how to start discord on mac and linux
            Some(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
            }) => {
                println!("Help requested!");
                match OS {
                    "windows" => {
                        let dir = home::home_dir().unwrap();
                        let full = format!("{}\\Desktop\\Discord.lnk", dir.to_str().unwrap());
                        Command::new("powershell")
                            .arg(full)
                            .spawn()
                            .expect("failed to open Discord");
                    }
                    "macos" => {
                        Command::new("open")
                            .arg("-a")
                            .arg("Discord")
                            .spawn()
                            .expect("failed to open Discord");
                    }
                    "linux" => {
                        Command::new("discord")
                            .spawn()
                            .expect("failed to open Discord");
                    }
                    &_ => todo!(),
                }
                config.set_time_left(
                    config.get_time_left("Discord".to_string())
                        + config.get_help_time("Discord".to_string()),
                    "Discord".to_string(),
                );
                gist_sync
                    .sync(config.gist.clone().unwrap(), config.apps.clone())
                    .unwrap();
            }
            Some(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                println!();
                exit(1);
            }
            _ => {
                println!("Help not requested!");
            }
        }
        let ps: String = match OS {
            "windows" => {
                let output = Command::new("powershell")
                    .arg("ps")
                    .arg("|")
                    .arg("Select-String")
                    .arg("-Pattern")
                    .arg("Discord")
                    .output()
                    .expect("Failed to execute powershell");
                String::from_utf8_lossy(&output.stdout).into()
            }
            _ => {
                let mut cmd = Command::new("sh");
                let output = cmd
                    .arg("-c")
                    .arg("ps -Axc | grep Discord")
                    .output()
                    .expect("Failed to execute sh");

                String::from_utf8_lossy(&output.stdout).into()
            }
        };
        // check if there is time left
        if config.get_time_left("Discord".to_string()) == 0 && !ps.is_empty() {
            io::stdout().flush().unwrap();

            println!("Time's up!");
            match OS {
                "windows" => Command::new("powershell")
                    .arg("kill")
                    .arg("-Name")
                    .arg("Discord")
                    .spawn()
                    .expect("failed to close Discord"),
                _ => Command::new("killall")
                    .arg("Discord")
                    .spawn()
                    .expect("failed to close Discord"),
            };
        } else if !ps.is_empty() {
            config.set_time_left(
                config.get_time_left("Discord".to_string()) - 2,
                "Discord".to_string(),
            );
            gist_sync
                .sync(config.gist.clone().unwrap(), config.apps.clone())
                .unwrap();
        }
    }
}

fn read_key(timeout: Duration) -> Option<KeyEvent> {
    struct RawModeGuard;
    impl Drop for RawModeGuard {
        fn drop(&mut self) {
            terminal::disable_raw_mode().unwrap();
        }
    }

    terminal::enable_raw_mode().unwrap();
    let _guard = RawModeGuard;
    let start = Instant::now();
    let mut offset = Duration::ZERO;
    while offset <= timeout && event::poll(timeout - offset).unwrap() {
        if let Event::Key(event) = event::read().unwrap() {
            return Some(event);
        }
        offset = start.elapsed();
    }
    None
}
