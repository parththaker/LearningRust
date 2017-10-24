use std::io;
fn main() {

    let mut old_time_string = String::new();
    let input = io::stdin();

    input.read_line(&mut old_time_string)
        .expect("Input not readable!!");

    let mut new_try = old_time_string.trim();
    let length = old_time_string.len();


    let mut time_without_stamp = &old_time_string[..length-2];
    let time_stamp = &old_time_string[length-2..].to_string();
    let mut time_to_change: i32 = old_time_string[..2].parse::<i32>().unwrap();

    if {time_stamp == "PM"} {
    time_to_change += 12;
    }
    time_to_change = time_to_change % 24;

    println!("{:02}{}", time_to_change, &old_time_string[2..length-2])

}
