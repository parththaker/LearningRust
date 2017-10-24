use std::io;
use std::u64;
fn main() {
    //println!("lets start!");

    let mut number_string = String::new();
    let input = io::stdin();

    input.read_line(&mut number_string)
        .expect("Input cannot be processed");

    let num_arr: Vec<u64> = number_string.split_whitespace()
        .map(|x| x.parse::<u64>().expect("Input number array cannot be parsed to u64"))
        .collect();

    let mut max_elem: u64 = 0;
    let mut min_elem: u64 = u64::pow(10, 10);
    let mut sum:u64 = 0;

    for i in 0..num_arr.len() {
        if {max_elem < num_arr[i]} {
            max_elem = num_arr[i];

        }
        if {min_elem > num_arr[i]} {
            min_elem = num_arr[i];
        }
        sum += num_arr[i];
    }

    println!("{} {}", sum-max_elem, sum-min_elem);
}
