use std::io;
use std::process;

fn main() {

    let mut first_row = String::new();
    let mut second_row = String::new();

    io::stdin().read_line(&mut first_row)
        .expect("First row input not processing");

    io::stdin().read_line(&mut second_row)
        .expect("Second row input not processing");

    let first_scores = first_row.split_whitespace()
        .map(|x| x.parse::<i32>().expect("First row input not convertable"))
        .collect::<Vec<i32>>();
    let second_scores = second_row.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Second row input not convertable"))
        .collect::<Vec<i32>>();

    if first_scores.len() != second_scores.len() {
        println!("Different sizes of rows. Exiting");
        process::exit(0);
    }

    let mut first_tot: u32 = 0;
    let mut second_tot : u32 = 0;

    for i in 0..first_scores.len() {
        if {first_scores[i] > second_scores[i]} {
            first_tot += 1;
        } else if {second_scores[i] > first_scores[i]} {
            second_tot +=1;
        }
    }

    println!("{} {}", first_tot, second_tot);
}
