use std::io::Read;

use crate::{play, info};
use colored::*;

#[derive(Debug)]
pub struct Config {
    pub threads: u32,
    pub method: play::Method,
}

impl Config {
    pub fn default() -> Self {
        println!("{}", "Using default config...".yellow().bold());

        Config {
            threads: auto_threads(),
            method: play::Method::Md5,
        }
    }

    pub fn from_file(path: &str) -> Self {
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("{}", 
                    format!("Failed to open config file `{}`,\n\tbecause `{}`", path, e)
                        .red().italic());
                return Config::default();
            }
        };
        
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(e) => {
                println!("{}", 
                    format!("Failed to read config file `{}`,\n\tbecause `{}`", path, e)
                        .red().italic());
                return Config::default();
            }
        };

        let toml_value: toml::Value = match toml::from_str(&contents) {
            Ok(toml_value) => toml_value,
            Err(e) => {
                println!("{}", 
                    format!("Failed to parse config file `{}`,\n\tbecause `{}`", path, e)
                        .red().italic());
                return Config::default();
            }
        };
        if toml_value.get("threads").is_none() {
            println!("{}", 
                format!("Failed to parse config file `{}`,\n\tbecause `threads` is not found", path)
                    .red().italic());
            return Config::default();
        }
        if toml_value.get("method").is_none() {
            println!("{}", 
                format!("Failed to parse config file `{}`,\n\tbecause `method` is not found", path)
                    .red().italic());
            return Config::default();
        }

        let threads = match toml_value["threads"].as_integer() {
            Some(threads) => {
                match threads {
                    t if t <= 0 => {
                        println!("{}", 
                        "Threads set to `auto` while `threads` equal or less than 0".red().italic());
                        println!("{}", "Use half of the machine's max threads as default".yellow().bold());
                        auto_threads()
                    },
                    t => {
                        if t as u32 > info::get_max_threads() {
                            println!("{}", 
                            format!("Threads must be less than {}",
                                info::get_max_threads()).red().italic());
                            return Config::default();
                        }
                        t as u32
                    }
                }
            },
            None => {
                println!("{}", 
                "Failed to parse `threads` from config file".red().italic());
                return Config::default();
            }
        };

        let method = match toml_value["method"].as_str() {
            Some(method) => {
                match method {
                    "md5" => {
                        play::Method::Md5
                    },
                    "sha3" => {
                        play::Method::Sha3
                    },
                    _ => {
                        println!("{}", 
                        "Failed to parse `method` from config file".red().italic());
                        println!("{}", "Use `md5` as default".yellow().bold());
                        play::Method::Md5
                    }
                }
            },
            None => {
                println!("{}", 
                "Failed to parse `method` from config file".red().italic());
                return Config::default();
            }
        };

        println!("{}", 
            format!("Using config file `{}`", path).yellow().bold());

        Config {
            threads,
            method,
        }
    }
}

fn auto_threads() -> u32 {
    if info::get_max_threads() / 2 < 1 {
        1
    } else {
        info::get_max_threads() / 2
    }
}
