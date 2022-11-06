use std::fs;
fn main() {
    println!("Hello, world!");
    let input_file = std::fs::read_to_string("1.txt").unwrap();
    let input_str = input_file.as_str();
    let number_increased =  part1(input_str);
    println!("the number of times the depth increased was: {}", number_increased);
}

fn part1(input: &str)-> String {
    input
    .lines()
    .fold((0, None), |(mut n, prev), line|{
        let depth = line.trim().parse::<i64>().unwrap();
        if let Some(prev) = prev {
            if depth > prev{
                n = n+1;
            }
            
        }
        (n, Some(depth))
    })
    .0
    .to_string()
}


#[test]
fn test_day1(){
    assert_eq!("7",
    part1("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"));
}