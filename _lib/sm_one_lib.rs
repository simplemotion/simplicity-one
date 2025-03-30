// This crate is a library
#![crate_type = "lib"]
// The library is named "sm_one_lib"
#![crate_name = "sm_one_lib"]

use std::error::Error;
use std::fs;

fn sm_one_version_private() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("version.txt")?;
    print!("Simplicity-One - lib - {}", message);
    Ok(())
}

pub fn sm_one_version() {
    let _ = sm_one_version_private();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
