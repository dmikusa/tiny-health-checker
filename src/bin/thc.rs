use anyhow::Result;
use tiny_health_checker::THC;

fn main() -> Result<()> {
    THC::new()?.exec()
}
