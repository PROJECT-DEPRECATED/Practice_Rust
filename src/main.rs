use std::io;
use rand::Rng;

fn main() {
    get_number()
}

fn get_number() {
    let mut fix_variable = String::new();
    println!("수를 입력하여 주십시오.");

    io::stdin().read_line(&mut fix_variable).expect("Can't read typed number");
    println!("입력받은 수: {}", fix_variable);
}