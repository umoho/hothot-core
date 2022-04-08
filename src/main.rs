mod play;

use colored::*;

const MOTD: &'static str = "Hot-hot core starting up...";

fn main() {
    println!("\n{}", MOTD.blue().bold());
    let start = std::time::Instant::now();

    ctrlc::set_handler(
        move || {
            println!("{}", "\nShutting down...".red().italic());

            let end = std::time::Instant::now();
            let elapsed = end.duration_since(start);
            println!("{}", format!(
                "Run for {} seconds", elapsed.as_secs_f32().round()
            ).yellow().italic());

            std::process::exit(0);
        }
    ).expect(format!("{}", "Failed to set Ctrl-C handler.".red().italic()).as_str());

    let running_threads = if get_max_threads() / 2 < 1 {
        1
    } else {
        get_max_threads() / 2
    };
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
