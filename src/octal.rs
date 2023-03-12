use std::io;

pub fn octal_input() {
    println!("input octal :");
    let mut octal = String::new();
    io::stdin().read_line(&mut octal).unwrap();

    if !octal.trim().chars().all(|c| {
        c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7'
    }) {
        println!("invalid input");
        return;
    }

    oct_to_bin(&octal);
    oct_to_dec(&octal);
    oct_to_hex(&octal)
}

fn oct_to_bin(octal: &str) {
    let dec = i64::from_str_radix(octal.trim(), 8).unwrap();
    let bin = format!("{:b}", dec);
    println!("binary : {}", bin)
}

fn oct_to_dec(octal: &str) {
    let dec = i64::from_str_radix(octal.trim(), 8).unwrap();
    println!("decimal : {}", dec)
}

fn oct_to_hex(octal: &str) {
    let dec = i64::from_str_radix(octal.trim(), 8).unwrap();
    let hex = format!("{:X}", dec);
    println!("hexa : {}", hex)
}
