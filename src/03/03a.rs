use regex::Regex;

fn main() {
    let regex_uncorrupted = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let regex_numbers = Regex::new(r"\d{1,3}").unwrap();
    let score = regex_uncorrupted
        .captures_iter(include_str!("input.txt"))
        .map(|captured| {
            //println!("{:?}",captured.get(0).unwrap().as_str());
            regex_numbers
                .captures_iter(captured.get(0).unwrap().as_str())
                .map(|digit| digit.get(0).unwrap().as_str().parse::<usize>().unwrap())
                .fold(1, |acc, x| acc * x)
        })
        .sum::<usize>();
    print!("{:?}",score)
}
