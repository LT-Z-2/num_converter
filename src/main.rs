use std::io;

// steps:
//      1) Ask for what type of base input will be (bin, dec, oct, hex)
//      2) get input
//      3) ask what base they would like to be returned
//      4) send input to whichever function
//      5) return output

fn main() {
    let mut input = String::new();

    println!("Please enter the type of base for input (bin, dec, hex):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read.");

    let input_base = input.trim();
    input.clear();

    println!("Please enter the value:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read.");

    let value = input.trim().to_string();
    input.clear();

    println!("Please enter the type of base for output (bin, dec, hex):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read.");

    let output_base = input.trim();

    let result = match (input_base, output_base) {
        ("dec", "bin") => dec_to_bin(value),
        ("dec", "hex") => dec_to_hex(value),
        ("dec", "dec") => dec_to_dec(value),
        ("bin", "dec") => bin_to_dec(value),
        ("bin", "hex") => bin_to_hex(value),
        ("bin", "bin") => bin_to_bin(value),
        ("hex", "dec") => hex_to_dec(value),
        ("hex", "bin") => hex_to_bin(value),
        ("hex", "hex") => hex_to_hex(value),
        _ => {
            println!("Invalid base conversion.");
            return;
        }
    };

    println!("The result is: {}", result);
}

fn dec_to_bin(input: String) -> String {
    let number: u32 = input.trim().parse().expect("Please type a valid number!");
    format!("{:b}", number)
}

fn dec_to_hex(input: String) -> String {
    let number: u32 = input.trim().parse().expect("Please type a valid number!");
    format!("{:x}", number)
}

fn dec_to_dec(input: String) -> String {
    input.trim().to_string()
}

fn bin_to_dec(input: String) -> String {
    u32::from_str_radix(input.trim(), 2)
        .map(|n| n.to_string())
        .expect("Please type a valid binary number!")
}

fn bin_to_hex(input: String) -> String {
    u32::from_str_radix(input.trim(), 2)
        .map(|n| format!("{:x}", n))
        .expect("Please type a valid binary number!")
}

fn bin_to_bin(input: String) -> String {
    input.trim().to_string()
}

fn hex_to_dec(input: String) -> String {
    u32::from_str_radix(input.trim(), 16)
        .map(|n| n.to_string())
        .expect("Please type a valid hexadecimal number!")
}

fn hex_to_bin(input: String) -> String {
    u32::from_str_radix(input.trim(), 16)
        .map(|n| format!("{:b}", n))
        .expect("Please type a valid hexadecimal number!")
}

fn hex_to_hex(input: String) -> String {
    input.trim().to_string()
}
