use clap::{Parser,Subcommand};
use std::fs;
use std::env;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    Scale {
        value: String
    },
    GetOnline {
    },
    GetAvailable {
    },
    GetOffline {}
}

fn get_available_count() -> u8 {
    count_cpu_file("/sys/devices/system/cpu/possible").unwrap()
}

fn get_offline_count() -> u8 {
    count_cpu_file("/sys/devices/system/cpu/offline").unwrap()
}

fn get_online_count() -> u8 {
    count_cpu_file("/sys/devices/system/cpu/online").unwrap()
}

fn count_cpu_file(path: &str) -> Result<u8, &str> {
    let contents = fs::read_to_string(path).unwrap();
    let ranges: Vec<&str> = contents.trim().split(",").collect();
    let mut count: u8 = 0;

    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();
        let len = parts.len();

        if len == 1 {
            count += 1;
            continue ;
        }

        if len == 2 {
            count += parts[1].parse::<u8>().unwrap() - parts[0].parse::<u8>().unwrap() + 1;
        }
    }

    Ok(count)
}

fn set_online(value: &str) -> Result<(), &str> {
    let v = value.parse::<u8>().unwrap();

    if v < 1 {
        return Err("You should at least set 1 CPU.");
    }

    for n in 1..get_available_count() {
        let path = format!("/sys/devices/system/cpu/cpu{}/online", n);
        let state: u8;

        if n < v {
            state = 1;
        } else {
            state = 0;
        }

         fs::write(path, format!("{}\n", state)).unwrap();
    }

    return Ok(());
}

fn is_root() -> bool {
    match env::var("USER") {
        Ok(uid) => uid == "root",
        _ => false
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::GetAvailable {} => {
            println!("{}", get_available_count());
        },
        Command::GetOnline {  } => {
            println!("{}", get_online_count());
        },
        Command::GetOffline {} => {  
            println!("{}", get_offline_count());
        },
        Command::Scale { value } => {
            if !is_root() {
                eprintln!("You need to run this command as root");

                return 
            }

            match set_online(&value) {
                Ok(()) => {
                    println!("Ok")
                },
                Err(err) => {
                    eprintln!("Failed to scale CPUs: {}", err);

                    return;
                },
            }
        },
    }
}

