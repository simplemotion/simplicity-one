use sm_one_lib::sm_one_version;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("version.txt")?;
    print!("Simplicity-One - bin - {}", message);
    sm_one_version();
    Ok(())
}
