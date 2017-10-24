
use std::io;

fn main() {

    let mut tot_num = String::new();
    io::stdin().read_line(&mut tot_num)
        .expect("Total entries input not able to process");

    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
        .expect("Input number array not able to process");

//    METHOD 1
    let num_vec = numbers.split_whitespace()
        .map(|x| x.parse::<i64>().expect("Number array not able to convert"));

    let mut sum: i64 = 0;

    for num in num_vec {
        sum += num;
    }

//    METHOD 2
    let second_sum: i64 = numbers.split_whitespace()
        .map(|x| x.parse::<i64>().expect("Not able to parse as i64"))
        .fold(0, |sum, i| sum+i);

    println!("{}", sum);

}
