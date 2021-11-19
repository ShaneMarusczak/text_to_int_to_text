use std::collections::HashMap;

fn main() {}

fn get_power(num: u32) -> u32 {
    if num == 0 {
        2
    } else {
        num * 3
    }
}

fn text_to_int(text_num: &str) -> usize {
    let mut num_words: HashMap<String, (usize, usize)> = HashMap::new();

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

    for (idx, word) in units.iter().enumerate() {
        num_words.insert(String::from(*word), (1, idx));
    }

    for (idx, word) in tens.iter().enumerate() {
        num_words.insert(String::from(*word), (1, idx * 10));
    }

    let num: usize = 10;

    for (idx, word) in scales.iter().enumerate() {
        num_words.insert(String::from(*word), (num.pow(get_power(idx as u32)), 0));
    }

    let mut current = 0;

    let mut result = 0;

    for word in text_num.split_whitespace() {
        if !num_words.contains_key(word) {
            panic!("Invalid word")
        }

        let (scale, increment) = num_words[word];

        current = current * scale + increment;

        if scale > 100 {
            result += current;
            current = 0;
        }
    }

    result + current
}
