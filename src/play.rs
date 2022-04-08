use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha3;
use rand::Rng;
use colored::*;

pub fn multi_thread(threads: u32) {
    for i in 0..threads {
        let thread = std::thread::spawn(move || {
            println!("{}", format!("Thread {} started...", i).cyan());
            one_thread();
        });

        if i == threads - 1 {
            thread.join().expect(
                format!("{}", "Failed to join thread.".red().bold().italic()).as_str());
        }
    }
}

fn one_thread() {
    loop {
        // make cpu busy
        md5(gen_num().to_string().as_str());
        sha3(gen_num().to_string().as_str());
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
