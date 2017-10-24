use std::io;

fn main() {

    let mut param = String::new();
    let input = io::stdin();

    input.read_line(&mut param)
        .expect("Input cannot be processed");

    let size_arr: Vec<i32> = param.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Input cannot be parsed to i32"))
        .collect();

    let size: i32 = size_arr[0]+1;

    for i in 1..size {
        for _ in 0..(size - i-1) {
            print!(" ");
        }
        for _ in 0..i {
            print!("#");
        }
        println!("");
    }
}
