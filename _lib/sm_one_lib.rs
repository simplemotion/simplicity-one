// This crate is a library
#![crate_type = "lib"]
// The library is named "sm_one_lib"
#![crate_name = "sm_one_lib"]

fn hello_private() {
    println!("sm-one - lib");
}

pub fn hello_sm_one() {
    hello_private();
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
