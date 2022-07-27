use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal,
};

use std::env;
use std::env::consts::OS;
use std::io::{self, Write};
use std::process::{exit, Command};
use std::thread;
use std::time::{Duration, Instant};
// Todo make window pop up on top of all other windows if help is requested and pause discord unrtill help is closed
// Todo make the interval for checking for Discord more often
fn main() {
    let mut day = chrono::Local::today();
    let mut time_left = 50;
    let mut help_time = 5; // this defines time for help peroid
    if env::args().len() > 1 {
        help_time = env::args().nth(1).unwrap().parse::<u64>().unwrap();
    }
    println!("{}", help_time);
    loop {
        // check if its a new day
        if day != chrono::Local::today() {
            println!("New day!");
            day = chrono::Local::today();
            time_left = 50;
        }
        println!("{}", time_left);
        println!("type 'h' for help");
        let help = read_key(Duration::from_secs(120));
        match help {
            // Todo figureout how to start discord on mac and linux
            Some(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
            }) => {
                println!("Help requested!");
                let dir = home::home_dir().unwrap();
                let full = format!("{}\\Desktop\\Discord.lnk", dir.to_str().unwrap());
                Command::new("powershell")
                    .arg(full)
                    .spawn()
                    .expect("failed to open Discord");
                thread::sleep(Duration::from_secs(help_time * 60));
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
        if time_left == 0 && !ps.is_empty() {
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
            time_left -= 2;
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
