mod play;
mod conf;
mod info;
mod cli;

use colored::*;

const MOTD: &str = "Hot-hot core starting up...";
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("\n");

    handler_registering();
    broadcast_registering(60.0);

    let config_path = cli::parse_args().config;
    let config = conf::Config::from_file(&config_path);

    println!("\n{}", MOTD.blue().bold());
    println!("{}\n{}",
        format!("\tAuthor: {}", AUTHORS).cyan(),
        format!("\tVersion: {}", VERSION).cyan());

    play::multi_thread(config);
}

fn handler_registering() {
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
    ).expect(format!("{}", "Failed to set Ctrl-C handler".red().italic()).as_str());
}

fn broadcast_registering(secs: f32) {
    info::broadcast(std::time::Duration::from_secs_f32(secs));
}
