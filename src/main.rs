use std::io;
use to_int_and_back::to;

fn main() {
    println!("Number text to integer");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // let num = to::int(&name);

    let i: isize = match name.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("AHHHH"),
    };

    let num = to::string(i);

    println!("{}", num);
}
