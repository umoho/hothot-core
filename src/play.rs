use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha3;
use rand::Rng;
use colored::*;

use crate::conf::{self, Config};

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Method {
    Md5,
    Sha3,
}

pub fn multi_thread(config: conf::Config) {
    let Config { threads, method } = config;

    println!("{}", "\nStarting...".green().italic());
    for i in 0..threads {
        let thread = std::thread::spawn(move || {
            println!("{}", format!("\tThread {} started...", i).cyan());
            one_thread(method);
        });

        if i == threads - 1 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("{}", format!("Spawned {} threads", i + 1).green().italic());
            thread.join().expect(
                format!("{}", "Failed to join thread".red().bold().italic()).as_str());
        }
    }
}

fn one_thread(method: Method) {
    loop {
        // make cpu busy
        match method {
            Method::Md5 => {
                md5(gen_num().to_string().as_str());
            },
            Method::Sha3 => {
                sha3(gen_num().to_string().as_str());
            },
        };
    }
}

fn md5(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(s);
    hasher.result_str()
}

fn sha3(s: &str) -> String {
    let mut hasher = sha3::Sha3::sha3_256();
    hasher.input_str(s);
    hasher.result_str()
}

fn gen_num() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>()
}
