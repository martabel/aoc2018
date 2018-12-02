use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn run(input: &str) {
    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let freq_count = contents.lines().count();
    let mut freq_moves: Vec<i32> = Vec::new();

    let mut freq: i32 = 0;
    for s in contents.lines() {
        //println!("{}", s);
        let val: i32 = s.parse().unwrap();
        freq_moves.push(val);
        freq += val;
    }
    println!("Part 1 Frequency {}", freq);

    // part 2
    let mut freqs = HashSet::new();
    let mut freq_move_idx = 0;
    // start with zero
    freq = 0;

    loop {
        freqs.insert(freq);

        freq += freq_moves[freq_move_idx];
        freq_move_idx += 1;
        if freq_move_idx >= freq_count {
            freq_move_idx = 0;
        }

        if freqs.contains(&freq) {
            break;
        }
    }

    println!("Part 2 First frequency twice {}", freq);
}
