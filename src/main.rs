use std::collections::HashMap;
use std::io;

fn main() {
    println!("Number text to integer");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let num = text_to_int(&name);

    println!("{}", num);
}

fn get_power(num: usize) -> u32 {
    if num == 0 {
        2
    } else {
        (num * 3) as u32
    }
}

fn text_to_int(text_num: &str) -> isize {
    let mut num_words: HashMap<String, (isize, isize)> = HashMap::new();

    let units = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let scales = ["hundred", "thousand", "million", "billion", "trillion"];

    num_words.insert(String::from("and"), (1, 0));

    for (index, word) in units.iter().enumerate() {
        num_words.insert(String::from(*word), (1, index as isize));
    }

    for (index, word) in tens.iter().enumerate() {
        num_words.insert(String::from(*word), (1, index as isize * 10));
    }

    let num: usize = 10;

    for (index, word) in scales.iter().enumerate() {
        num_words.insert(String::from(*word), (num.pow(get_power(index)) as isize, 0));
    }

    let mut current = 0;

    let mut result = 0;

    let mut multipler = 1;

    for word in text_num.split_whitespace() {
        if word.eq("negative") {
            multipler = -1;
        } else if !num_words.contains_key(word) {
            panic!("Invalid word")
        } else {
            let (scale, increment) = num_words[word];

            current = current * scale + increment;
            if scale > 100 {
                result += current;
                current = 0;
            }
        }
    }

    (result + current) * multipler
}
