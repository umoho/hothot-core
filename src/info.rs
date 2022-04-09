use std::thread;

use colored::*;
use systemstat::Platform;

pub fn broadcast(duration: std::time::Duration) {
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(105));
        loop {
            let get_cpu_temp = get_cpu_temp();
            match get_cpu_temp {
                Ok(t) => {
                    println!("{}", format!(
                        "\nCPU temperature: {}°C", t).green());
                },
                Err(e) => {
                    println!("{}", format!(
                        "\nCPU temperature unknown,\n\tbecause `{}`", e).red().italic());
                }
            }
            thread::sleep(duration);
        }
    });
}

// get machine max threads
pub fn get_max_threads() -> u32 {
    let num_cpus = num_cpus::get();
    if num_cpus <= 0 {
        panic!("{}", "Failed to get number of CPUs".red().bold().italic());
    }
    num_cpus as u32
}

// get CPU temperature
pub fn get_cpu_temp() -> Result<f32, std::io::Error> {
    let sys = systemstat::System::new();
    let temp = sys.cpu_temp()?;
    Ok(temp)
}
