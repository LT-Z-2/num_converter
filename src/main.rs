use std::io;

// steps:
//      1) Ask for what type of base input will be (bin, dec, oct, hex)
//      2) get input
//      3) ask what base they would like to be returned
//      4) send input to whichever function
//      5) return output



fn main() {

    let mut input = String::new();

    println!("Please enter ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read.");


}

fn bin_to_dec(mut input: String) -> u32 {
    return 0;
}

fn oct_to_dec(mut input: String) -> u32 {
    return 0;
}

fn hex_to_dec(mut input: String) -> u32 {
    return 0;
}
