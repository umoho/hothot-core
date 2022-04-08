mod play;

use colored::*;

const MOTD: &'static str = "Hot-hot core starting up...";

fn main() {
    println!("\n{}", MOTD.blue().bold());

    ctrlc::set_handler(
        move || {
            println!("{}", "\nShutting down...".red().italic());
            std::process::exit(0);
        }
    ).expect(format!("{}", "Failed to set Ctrl-C handler.".red().italic()).as_str());

    let running_threads = get_max_threads() / 2;
    println!("{}", format!("Running {} threads...", running_threads).green().italic());
    play::multi_thread(running_threads);
}

// get machine max threads
fn get_max_threads() -> u32 {
    let num_cpus = num_cpus::get();
    if num_cpus <= 0 {
        panic!("{}", "Failed to get number of CPUs.".red().bold().italic());
    }
    num_cpus as u32
}
