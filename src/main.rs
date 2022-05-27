use chrono;
use std::env::consts::OS;
use std::io::{self, Write};
use std::process::Command;
use std::thread;

// Todo make window pop up on top of all other windows if help is requested and pause discord unrtill help is closed
fn main() {
    let day = chrono::Local::today();
    let mut time_left = 0;
    loop {
        // check if its a new day
        if day != chrono::Local::today() {
            println!("New day!");
            time_left = 120;
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
                let output = Command::new("ps")
                    .arg("-ax")
                    .arg("|")
                    .arg("grep")
                    .arg("Discord")
                    .output()
                    .expect("Failed to execute ps");
                String::from_utf8_lossy(&output.stdout).into()
            }
        };
        // check if there is time left
        if time_left == 0 && !ps.is_empty() {
            loop {
                let mut help = String::new();
                print!("Discord is running!\nIf you are using it for help, type \"help\" to continue using it: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut help)
                    .expect("Failed to read line");
                if help.trim() == "help" {
                    // wait 5 minutes
                    println!("Waiting 5 minutes...");
                    thread::sleep(std::time::Duration::from_secs(300));
                    continue;
                } else {
                    break;
                }
            }
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
            time_left -= 5;
        }
        // wait for 2 minutes
        thread::sleep(std::time::Duration::from_secs(120));
    }
}
