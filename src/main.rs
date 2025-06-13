use clap::Parser;
use clap_derive::Parser;
use serde::Deserialize;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, trailing_var_arg = true)]
struct Cli {
    /// Whether to shut down the system after command completion
    #[arg(short, long)]
    shutdown: bool,

    #[arg(required = true)]
    command: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    app_token: String,
    user_key: String,
}

fn main() {
    let directory = directories::ProjectDirs::from("", "", "run_notify")
        .expect("Couldn't find app directory")
        .config_dir()
        .to_path_buf();

    let mut path = directory.clone();
    path.push("config.toml");

    if !path.is_file() {
        match create_dir_all(directory) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to create directories to config file: {e}");
            }
        };

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => {
                panic!("Cannot create config file at {:?}: {e}", path);
            }
        };

        match file.write_all(b"app_token=\"\"\nuser_key=\"\"") {
            Ok(_) => {}
            Err(e) => {
                panic!("Cannot write to config file at {:?}: {e}", path);
            }
        }

        println!(
            "Config file generated at {:?}. Fill it out before running again.",
            path
        );
        return;
    }

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot read config file at {:?}", path));

    let mut config_string = String::new();

    file.read_to_string(&mut config_string)
        .expect("Couldn't read config file into string");

    let config: Config = match toml::from_str(&config_string) {
        Ok(config) => config,
        Err(e) => {
            panic!("Failed to parse config toml: {e}");
        }
    };

    let cli = Cli::parse();

    let command = &cli.command[0];
    let command_args = &cli.command[1..];

    let status = Command::new(command).args(command_args).status();

    match status {
        Ok(status) => {
            let mut params = vec![
                ("token".to_string(), config.app_token),
                ("user".to_string(), config.user_key),
            ];

            if status.success() {
                println!("Command succeeded.");

                params.push((
                    "message".to_string(),
                    format!(
                        "[{}] {}",
                        gethostname::gethostname().to_string_lossy(),
                        cli.command.join(" ")
                    ),
                ));

                params.push(("title".to_string(), "Command Succeeded".to_string()));
            } else {
                eprintln!("Command failed with status: {}", status);

                params.push((
                    "message".to_string(),
                    format!(
                        "[{}] {}",
                        gethostname::gethostname().to_string_lossy(),
                        cli.command.join(" ")
                    ),
                ));

                params.push(("title".to_string(), "Command Failed".to_string()));
            }

            let client = reqwest::blocking::Client::new();
            let res = client
                .post("https://api.pushover.net/1/messages.json")
                .form(&params)
                .send();

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Sent notification");
                    } else {
                        println!(
                            "Invalid notification. Something is probably configured wrong: {:?}",
                            response.text().unwrap_or("No Text Response".to_string())
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Failed to send notification: {e}");
                }
            }

            if cli.shutdown {
                println!("Shutting down in 10 seconds");

                sleep(Duration::from_secs(10));

                match system_shutdown::shutdown() {
                    Ok(_) => {
                        println!("Shutting down");
                    }
                    Err(e) => {
                        eprintln!("Failed to shutdown: {e}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {e}");
        }
    }
}
