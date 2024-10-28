use std::{env, process};

use tiny_health_checker::THC;

fn main() {
    if let Err(err) = THC::new(&env::args().collect::<Vec<String>>()).exec() {
        eprintln!("Error:");
        eprintln!("{err}");
        eprintln!();
        process::exit(1);
    }
}
