use std::io;
use to_int_and_back::to;

fn main() {
    println!("Number Conversion");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    match name.trim().parse::<isize>() {
        Ok(num) => println!("{}", to::string(num)),
        Err(_) => to_int(&name),
    };
}

fn to_int(old_name: &str) {
    let mut new_name = String::from(old_name);
    let mut retry = false;
    loop {
        if retry {
            io::stdin()
                .read_line(&mut new_name)
                .expect("Failed to read line");
        }
        match to::float(&new_name.trim()) {
            Ok(n) => {
                println!("{}", n);
                break;
            }
            Err(e) => {
                println!("{}", e);
                println!("Try Again? y/n");
                new_name.clear();

                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read line");
                if new_name.trim().eq("y") {
                    retry = true;
                    new_name.clear();
                } else {
                    break;
                }
            }
        }
    }
}
