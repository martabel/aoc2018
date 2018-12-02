use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct LineDiffResult {
    one: String,
    two: String,
    result: String,
    score: u32,
}

pub fn run(input: &str) {
    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // PART 1

    let mut twice_counter = 0;
    let mut thrice_counter = 0;

    for line in contents.lines() {
        let mut letter_counter: HashMap<char, i32> = HashMap::new();
        for c in line.chars() {
            *letter_counter.entry(c).or_insert(0) += 1;
        }

        if letter_counter.values().any(|&x| x == 2) {
            twice_counter += 1;
        }

        if letter_counter.values().any(|&x| x == 3) {
            thrice_counter += 1;
        }
    }

    println!(
        "Twice: {} Thirce: {} Checksum: {}",
        twice_counter,
        thrice_counter,
        twice_counter * thrice_counter
    );

    // PART 2

    // abcde
    // fghij 0
    // klmno 0 0
    // pqrst 0 0 0
    // fguij 0 4 0 0
    // axcye 3 0 0 0 0
    // wvxyz 0 0 0 0 0 1

    let mut line_list: Vec<LineDiffResult> = Vec::new();

    for (line_idx, line) in contents.lines().enumerate() {
        for (line_diff_idx, line_diff) in contents.lines().enumerate() {
            if line_diff_idx > line_idx {
                let mut line_pair = LineDiffResult {
                    one: line.to_string(),
                    two: line_diff.to_string(),
                    result: String::new(),
                    score: 0,
                };
                line_pair.two = line_diff.to_string();
                let mut line_diff_iter = line_diff.chars();
                for line_letter in line.chars() {
                    if line_letter == line_diff_iter.next().unwrap() {
                        line_pair.score += 1;
                        line_pair.result.push(line_letter);
                    }
                }
                line_list.push(line_pair);
            }
        }
    }

    let mut biggest_line_pair: &LineDiffResult = &LineDiffResult {
        one: String::new(),
        two: String::new(),
        result: String::new(),
        score: 0,
    };

    // Iterate over everything.
    for line_pair in &line_list {
        if line_pair.score > biggest_line_pair.score {
            biggest_line_pair = line_pair;
        }
    }

    println!(
        "{} {} {} {}",
        biggest_line_pair.one,
        biggest_line_pair.two,
        biggest_line_pair.score,
        biggest_line_pair.result
    );

    let mut result_string = String::from("");

    let mut line_one_iter = biggest_line_pair.one.chars();

    let mut line_two_iter = biggest_line_pair.two.chars();

    loop {
        match line_one_iter.next() {
            None => break,
            Some(letter) => {
                if letter == line_two_iter.next().unwrap() {
                    result_string.push(letter);
                } else {
                    result_string.push(' ');
                }
            }
        }
    }

    println!(
        "{} {} {}",
        result_string,
        result_string,
        str::replace(result_string.as_str(), " ", "")
    );
}
