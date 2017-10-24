use std::io;

fn main() {

    let mut total_numbers = String::new();

    let input = io::stdin();
    input.read_line(&mut total_numbers)
        .expect("Total number of entires cannot be processed");

    let total: Vec<i32> = total_numbers.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Cannot parse total number of entries to i32"))
        .collect();

    let total_n = total[0];

    let mut numbers = String::new();

    input.read_line(&mut numbers)
        .expect("Input number array cannot be processed");
    let num_arr: Vec<i32> = numbers.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Cannot parse input array to i32"))
        .collect();

    let each_frac: f32 = 1.0/(total_n as f32);
    let mut pos_frac: f32 = 0.0;
    let mut neg_frac: f32 = 0.0;
    let mut zero_frac: f32 = 0.0;

    for num in num_arr {
        if {num > 0} {
            pos_frac += each_frac;
        } else if {num <0} {
            neg_frac += each_frac;
        } else {
            zero_frac += each_frac;
        }
    }

    println!("{}\n{}\n{}", pos_frac, neg_frac, zero_frac);
}
