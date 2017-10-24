use std::io;
use std::u64;
fn main() {

    let mut number_string = String::new();
    let input = io::stdin();

    let mut scrap = String::new();

    input.read_line(&mut scrap)
        .expect("Total number of entries cannot be processed");

    input.read_line(&mut number_string)
        .expect("Input number array cannot be processed");

    let num_arr: Vec<u64> = number_string.split_whitespace()
        .map(|x| x.parse::<u64>().expect("Input number array cannot be parsed to u64"))
        .collect();

    let mut max_elem: u64 = 0;
    let mut sum:u64 = 0;

    for i in 0..num_arr.len() {
        if {max_elem < num_arr[i]} {
            max_elem = num_arr[i];

        }
    }

    for i in 0..num_arr.len() {
        if {max_elem == num_arr[i]} {
            sum += 1;

        }
    }

    println!("{}", sum);
}
