use std::io;
mod binary;
mod decimal;
mod hexa;
mod octal;

fn main() {
    println!("select with number");
    println!("1. binary to another");
    println!("2. octal to another");
    println!("3. decimal to another");
    println!("4. hexa to another");
    println!("5. exit");

    loop {
        println!("what are your options ?");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("failed");
        let int_number = number.trim().parse::<i32>().unwrap();

        if int_number == 1 {
            binary::binary_input();
        } else if int_number == 2 {
            octal::octal_input();
        } else if int_number == 3 {
            decimal::decimal_input();
        } else if int_number == 4 {
            hexa::hex_input()
        } else if int_number == 5 {
            println!("bye...");
            break;
        } else {
            println!("invalid input");
            break;
        }
    }
}
