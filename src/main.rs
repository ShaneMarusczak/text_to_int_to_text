use std::{collections::HashMap, io};

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
    let mut num_words: HashMap<&str, (isize, isize)> = HashMap::new();

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

    num_words.insert("and", (1, 0));

    let all_words = [
        "and",
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
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety",
        "hundred",
        "thousand",
        "million",
        "billion",
        "trillion",
    ];

    for (index, word) in units.iter().enumerate() {
        num_words.insert(*word, (1, index as isize));
    }

    for (index, word) in tens.iter().enumerate() {
        num_words.insert(*word, (1, index as isize * 10));
    }

    let num: usize = 10;

    for (index, word) in scales.iter().enumerate() {
        num_words.insert(*word, (num.pow(get_power(index)) as isize, 0));
    }

    let mut current = 0;

    let mut result = 0;

    let mut multipler = 1;

    for word in text_num.split_whitespace() {
        if min_distance(word, "negative") < 3 {
            if multipler == 1 {
                multipler = -1;
            } else {
                panic!("Invalid input");
            }
        } else if !all_words.contains(&word) {
            let word = find_possible_matches(&word, &all_words);

            let (scale, increment) = num_words[word.as_str()];

            current = current * scale + increment;
            if scale > 100 {
                result += current;
                current = 0;
            }
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

fn find_possible_matches(word: &str, words: &[&str]) -> String {
    let mut min_dist = 9999;
    let mut final_string: String = String::from("");

    for w in words {
        let distance = min_distance(*w, word);

        if distance < min_dist {
            min_dist = distance;
            final_string = String::from(*w);
        }
    }
    if min_dist > 1 {
        if min_distance(word, "negative") < 5 {
            println!("Did you mean negative?");
        } else {
            println!("Did you mean {}?", final_string);
        }
        panic!("Invalid input")
    }
    return final_string;
}

fn min_distance(word1: &str, word2: &str) -> i32 {
    let (word1, word2) = (word1.as_bytes(), word2.as_bytes());

    let mut dist = Vec::with_capacity(word2.len() + 1);

    for j in 0..=word2.len() {
        dist.push(j)
    }

    let mut prev_dist = dist.clone();

    for i in 1..=word1.len() {
        for j in 0..=word2.len() {
            if j == 0 {
                dist[j] += 1;
            } else if word1[i - 1] == word2[j - 1] {
                dist[j] = prev_dist[j - 1];
            } else {
                dist[j] = dist[j].min(dist[j - 1]).min(prev_dist[j - 1]) + 1;
            }
        }
        prev_dist.copy_from_slice(&dist);
    }
    dist[word2.len()] as i32
}
