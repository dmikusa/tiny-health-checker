use std::process;

use tiny_health_checker::THC;

fn main() {
    if let Err(err) = THC::new().exec() {
        eprintln!("Error:");
        eprintln!("{err}");
        eprintln!();
        process::exit(1);
    }
}
