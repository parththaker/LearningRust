use std::io;
fn main() {

    let mut old_time_string = String::new();
    let input = io::stdin();

    input.read_line(&mut old_time_string)
        .expect("Input cannot be processed");

    let new_try = old_time_string.trim();

    let length = old_time_string.len();

    let time_stamp = &new_try[length-2..].to_string();

    let mut time_to_change: i32 = new_try[..2].parse::<i32>().unwrap();

    if {time_stamp == "PM"}
        {
        if{time_to_change != 12}
            {
            time_to_change += 12;
        }
    }
    else
    {
        if{time_to_change == 12}
            {
            time_to_change += 12;
            }
    }
    time_to_change = time_to_change % 24;

    println!("{:02}{}", time_to_change, &new_try[2..length-2])
}


