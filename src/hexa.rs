use std::io;

pub fn hex_input() {
    println!("input hexa :");
    let mut hex = String::new();
    io::stdin().read_line(&mut hex).unwrap();

    if !hex.trim().chars().all(|c| {
        c == '0'
            || c == '1'
            || c == '2'
            || c == '3'
            || c == '4'
            || c == '5'
            || c == '6'
            || c == '7'
            || c == '8'
            || c == '9'
            || c == 'A'
            || c == 'B'
            || c == 'C'
            || c == 'D'
            || c == 'E'
            || c == 'F'
    }) {
        println!("invalid input");
        return;
    }

    hex_to_bin(&hex);
    hex_to_oct(&hex);
    hex_to_dec(&hex);
}

fn hex_to_bin(hex: &str) {
    let hex = u32::from_str_radix(hex.trim(), 16).unwrap();
    let bin = format!("{:b}", hex);
    println!("binary : {}", bin);
}

fn hex_to_oct(hex: &str){
    let hex = u32::from_str_radix(hex.trim(), 16).unwrap();
    let oct = format!("{:o}", hex);
    println!("octal : {}", oct)
}

fn hex_to_dec(hex: &str){    
    let dec = u32::from_str_radix(hex.trim(), 16).unwrap();
    println!("decimal : {}", dec)
}