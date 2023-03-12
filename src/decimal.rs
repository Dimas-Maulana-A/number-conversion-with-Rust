use std::io;

pub fn decimal_input() {
    println!("input decimal");
    let mut dec = String::new();
    io::stdin().read_line(&mut dec).unwrap();

    if !dec.trim().chars().all(|c| {
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
    }) {
        println!("invalid input");
        return;
    }

    dec_to_bin(&dec);
    dec_to_oct(&dec);
    dec_to_hex(&dec);
}

fn dec_to_bin(dec: &str) {
    let dec = u32::from_str_radix(&dec.trim(), 10).unwrap();
    let bin = format!("{:b}", dec);
    println!("binary : {}", bin)
}

fn dec_to_oct(dec: &str) {
    let dec = u32::from_str_radix(&dec.trim(), 10).unwrap();
    let oct = format!("{:o}", dec);
    println!("octal : {}", oct)
}

fn dec_to_hex(dec: &str) {
    let dec = u32::from_str_radix(&dec.trim(), 10).unwrap();
    let hex = format!("{:X}", dec);
    println!("hexa : {}", hex)
}
