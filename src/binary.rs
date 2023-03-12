use std::io;

pub fn binary_input() {
    println!("input binary :");
    let mut binary = String::new();
    io::stdin().read_line(&mut binary).unwrap();
    
    if !binary.trim().chars().all(|c| c == '0' || c == '1') {
        println!("invalid input");
        return;
    }
    
    bin_to_oct(&binary);
    bin_to_dec(&binary);
    bin_to_hex(&binary);
}

fn bin_to_oct(binary: &str) {
    let bin = u32::from_str_radix(binary.trim(), 2).unwrap();
    let oct = format!("{:o}", bin);
    println!("octal : {}", oct)
}

fn bin_to_dec(binary: &str) {
    let bin = u32::from_str_radix(&binary.trim(), 2).unwrap();
    println!("decimal : {}", bin)
}

fn bin_to_hex(binary: &str) {
    let bin = u32::from_str_radix(binary.trim(), 2).unwrap();
    let hex = format!("{:X}", bin);
    println!("hexa : {}", hex)
}
