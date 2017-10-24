use std::io;
use std::io::BufRead;

fn main() {

    let mut tot_num = String::new();
    let iostdin = io::stdin();

    iostdin.read_line(&mut tot_num)
        .expect("Cannot read the size of matrix");

    let num: Vec<i32> = tot_num.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Unable to parse as i32"))
        .collect();

    let mat_size: i32 = num[0];

    let all_lines = iostdin.lock().lines();
    let arr: Vec<Vec<i32>> = all_lines.take(mat_size as usize)
        .map(|l| l.unwrap().split_whitespace()
            .map(|x| x.parse::<i32>().expect("Unable to parse as i32"))
            .collect())
        .collect();

    let mut first_diag: i32 = 0;
    let mut second_diag: i32 = 0;

    let mut index1:usize;
    let mut index2:usize;

    for i in 0..mat_size {
        index1 = i as usize;
        index2 = (mat_size -1-i) as usize;
        first_diag += arr[index1][index1];
        second_diag += arr[index1][index2];
    }

    let abs = (first_diag - second_diag).abs();
    println!("{}", abs);

}
